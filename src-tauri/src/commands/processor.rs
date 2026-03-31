//! Shared packet processing pipeline.
//!
//! `PacketProcessor` encapsulates the full processing pipeline used by both
//! PCAP import and live capture: protocol identification → deep parse →
//! connection tracking → topology building → signature matching.

use std::collections::{HashMap, HashSet};

use gm_analysis::{ConnectionStats, PatternAnomaly};
use gm_capture::ParsedPacket;
use gm_db::{GeoIpLookup, OuiLookup};
use gm_parsers::{IcsProtocol, RedundancyInfo};
use gm_signatures::SignatureEngine;
use gm_topology::TopologyBuilder;

use super::{
    handlers::{
        bacnet::BacnetHandler, dnp3::Dnp3Handler, enip::EnipHandler, iec104::Iec104Handler,
        modbus::ModbusHandler, profinet::ProfinetDcpHandler, s7::S7Handler,
    },
    infer_device_type, AssetInfo, AssetSignatureMatch, ConnectionInfo, DeepParseInfo,
    PacketSummary,
};
use super::protocol_handler::{ProcessorCore, ProcessorOutput, ProtocolHandler};


/// Processes packets through the full pipeline:
/// protocol identification → deep parse → connection tracking → topology building.
///
/// Protocol-specific accumulation is delegated to [`ProtocolHandler`] impls in
/// `handlers/`. To add a new protocol: implement `ProtocolHandler`, register the
/// handler in `PacketProcessor::new()`, and add a `DeepParseResult` arm if needed.
///
/// Used by both PCAP import and live capture.
pub struct PacketProcessor {
    /// Shared cross-protocol packet state.
    core: ProcessorCore,

    pub topo_builder: TopologyBuilder,

    /// Per-protocol accumulators. Each handler is called for every packet.
    handlers: Vec<Box<dyn ProtocolHandler>>,

    pub total_packets: u64,
}

impl PacketProcessor {
    pub fn new() -> Self {
        let handlers: Vec<Box<dyn ProtocolHandler>> = vec![
            Box::new(ModbusHandler::default()),
            Box::new(Dnp3Handler::default()),
            Box::new(EnipHandler::default()),
            Box::new(S7Handler::default()),
            Box::new(BacnetHandler::default()),
            Box::new(Iec104Handler::default()),
            Box::new(ProfinetDcpHandler::default()),
        ];
        Self {
            core: ProcessorCore::default(),
            topo_builder: TopologyBuilder::new(),
            handlers,
            total_packets: 0,
        }
    }

    /// Process a single packet through the pipeline.
    pub fn process_packet(&mut self, packet: &ParsedPacket) {
        if let Some(deep_result) = self
            .core
            .process(packet, &mut self.topo_builder, &mut self.total_packets)
        {
            for handler in &mut self.handlers {
                handler.process(packet, &deep_result);
            }
        }
    }

    /// Build deep parse info by delegating to each registered handler, then
    /// appending LLDP and SNMP data (which are not routed through ProtocolHandler
    /// because they arrive on non-IP-layer frames).
    pub fn build_deep_parse_info(&self) -> HashMap<String, DeepParseInfo> {
        let mut output = ProcessorOutput::default();

        // Each handler populates its protocol-specific field.
        for handler in &self.handlers {
            handler.finalize(&mut output);
        }

        self.core.populate_core_deep_parse(&mut output.deep_parse);
        output.deep_parse
    }

    /// Collect all observed redundancy protocol frames as a flat list.
    ///
    /// Returns one `RedundancyInfo` per unique source MAC (last-frame-wins).
    pub fn build_redundancy_info(&self) -> Vec<RedundancyInfo> {
        self.core.build_redundancy_info()
    }

    /// Run signature matching and build the final asset list.
    ///
    /// Requires references to the SignatureEngine, OUI lookup, and GeoIP lookup.
    pub fn build_assets(
        &self,
        engine: &SignatureEngine,
        deep_parse_info: &HashMap<String, DeepParseInfo>,
        oui_lookup: &OuiLookup,
        geoip_lookup: &GeoIpLookup,
    ) -> (Vec<AssetInfo>, HashMap<String, Vec<AssetSignatureMatch>>) {
        // Run signature matching per device
        let mut sig_results: HashMap<String, Vec<AssetSignatureMatch>> = HashMap::new();
        for (ip, packets) in &self.core.ip_packets {
            let matches = engine.match_device_packets(packets);
            if !matches.is_empty() {
                sig_results.insert(
                    ip.clone(),
                    matches
                        .into_iter()
                        .map(|m| AssetSignatureMatch {
                            signature_name: m.signature_name,
                            confidence: m.confidence,
                            vendor: m.vendor,
                            product_family: m.product_family,
                            device_type: m.device_type,
                            role: m.role,
                        })
                        .collect(),
                );
            }
        }

        // Build assets
        let all_ips: HashSet<String> = self.core.asset_protocols.keys().cloned().collect();
        let mut assets: Vec<AssetInfo> = Vec::new();

        for ip in &all_ips {
            let protocols: Vec<IcsProtocol> = self
                .core.asset_protocols
                .get(ip)
                .map(|s| s.iter().copied().collect())
                .unwrap_or_default();

            let is_server = self.core.server_ips.contains(ip);
            let mut device_type = infer_device_type(&protocols, is_server);

            let sig_matches = sig_results.get(ip).cloned().unwrap_or_default();
            let best_match = sig_matches.first();

            let mut confidence = best_match.map(|m| m.confidence).unwrap_or(
                if protocols.iter().any(|p| *p != IcsProtocol::Unknown) {
                    1
                } else {
                    0
                },
            );

            let mut vendor = best_match.and_then(|m| m.vendor.clone());
            let mut product_family = best_match.and_then(|m| m.product_family.clone());

            // OUI vendor lookup from MAC address
            let mac = self.core.asset_macs.get(ip);
            let oui_vendor = mac.and_then(|m| oui_lookup.lookup(m).map(|v| v.to_string()));

            // If no signature vendor but OUI found, use OUI vendor + confidence 3
            if vendor.is_none() {
                if let Some(ref oui_v) = oui_vendor {
                    vendor = Some(oui_v.clone());
                    if confidence < 3 {
                        confidence = 3;
                    }
                }
            }

            // Deep parse Device ID (FC 43/14) overrides with confidence 5
            if let Some(dp_info) = deep_parse_info.get(ip) {
                if let Some(ref modbus) = dp_info.modbus {
                    if let Some(ref dev_id) = modbus.device_id {
                        confidence = 5;
                        if let Some(ref vn) = dev_id.vendor_name {
                            vendor = Some(vn.clone());
                        }
                        let pf_parts: Vec<&str> = [
                            dev_id.product_code.as_deref(),
                            dev_id.product_name.as_deref(),
                            dev_id.model_name.as_deref(),
                        ]
                        .iter()
                        .filter_map(|&x| x)
                        .collect();
                        if !pf_parts.is_empty() {
                            product_family = Some(pf_parts.join(" "));
                        }
                    }
                }
            }

            if let Some(m) = best_match {
                if let Some(ref sig_device_type) = m.device_type {
                    if m.confidence >= 3 {
                        device_type = sig_device_type.clone();
                    }
                }
            }

            // LLDP enrichment (confidence 4 — better than OUI/port, lower than deep parse)
            let mut hostname: Option<String> = None;
            if let Some(mac_addr) = self.core.asset_macs.get(ip) {
                if let Some(lldp) = self.core.lldp_by_mac.get(mac_addr) {
                    if let Some(ref sn) = lldp.system_name {
                        hostname = Some(sn.clone());
                    }
                    if vendor.is_none() {
                        if let Some(ref lv) = lldp.vendor {
                            vendor = Some(lv.clone());
                            if confidence < 4 {
                                confidence = 4;
                            }
                        }
                    }
                    if product_family.is_none() {
                        if let Some(ref lm) = lldp.model {
                            product_family = Some(lm.clone());
                        }
                    }
                    // If LLDP capabilities indicate bridge-only, classify as switch
                    if let (Some(cap), Some(en)) = (lldp.capabilities, lldp.enabled_capabilities) {
                        use gm_parsers::lldp::caps;
                        let active = if en != 0 { en } else { cap };
                        let is_bridge = active & caps::BRIDGE != 0;
                        let is_router = active & caps::ROUTER != 0;
                        if is_bridge && !is_router && device_type == "unknown" {
                            device_type = "switch".to_string();
                        }
                        if is_router && device_type == "unknown" {
                            device_type = "router".to_string();
                        }
                    }
                }
            }

            // GeoIP enrichment
            let is_public_ip = GeoIpLookup::is_public_ip(ip);
            let country = geoip_lookup.lookup_country(ip);

            assets.push(AssetInfo {
                id: ip.clone(),
                ip_address: ip.clone(),
                mac_address: self.core.asset_macs.get(ip).cloned(),
                hostname,
                device_type,
                vendor,
                protocols: protocols
                    .iter()
                    .map(|p| format!("{:?}", p).to_lowercase())
                    .collect(),
                first_seen: self.core.asset_first_seen.get(ip).cloned().unwrap_or_default(),
                last_seen: self.core.asset_last_seen.get(ip).cloned().unwrap_or_default(),
                notes: String::new(),
                purdue_level: None,
                tags: Vec::new(),
                packet_count: *self.core.asset_packet_counts.get(ip).unwrap_or(&0),
                confidence,
                product_family,
                signature_matches: sig_matches,
                oui_vendor,
                country,
                is_public_ip,
            });
        }

        // Sort: OT devices first, then by packet count descending
        assets.sort_by(|a, b| {
            let a_ot = a.device_type != "it_device" && a.device_type != "unknown";
            let b_ot = b.device_type != "it_device" && b.device_type != "unknown";
            b_ot.cmp(&a_ot).then(b.packet_count.cmp(&a.packet_count))
        });

        (assets, sig_results)
    }

    /// Finalize connections with origin file tracking.
    pub fn get_connections(&mut self) -> Vec<ConnectionInfo> {
        self.core.finalize_connections()
    }

    /// Get a snapshot of packet summaries.
    pub fn get_packet_summaries(&self) -> HashMap<String, Vec<PacketSummary>> {
        self.core.packet_summaries.clone()
    }

    /// Get protocols detected so far.
    pub fn get_protocols_detected(&self) -> Vec<String> {
        self.core.protocols_detected()
    }

    /// Compute per-connection-pair statistics and detect pattern anomalies.
    ///
    /// Returns `(stats, anomalies)` derived from Welford accumulators.
    /// Safe to call multiple times — no mutable state in PatternAnalyzer.
    pub fn build_pattern_results(&mut self) -> (Vec<ConnectionStats>, Vec<PatternAnomaly>) {
        self.core.pattern_results()
    }
}
