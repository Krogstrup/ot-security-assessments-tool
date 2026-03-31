//! Shared packet processing pipeline.
//!
//! `PacketProcessor` encapsulates the full processing pipeline used by both
//! PCAP import and live capture: protocol identification → deep parse →
//! connection tracking → topology building → signature matching.

use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use gm_analysis::{ConnectionStats, PatternAnalyzer, PatternAnomaly};
use gm_capture::ParsedPacket;
use gm_constants::is_ot_server_port;
use gm_db::{GeoIpLookup, OuiLookup};
use gm_parsers::{
    deep_parse, identify_protocol, parse_lldp, parse_redundancy, parse_snmp_response,
    DeepParseResult, IcsProtocol, LldpInfo, RedundancyInfo, SnmpDeviceInfo,
};
use gm_signatures::{PacketData, SignatureEngine};
use gm_topology::TopologyBuilder;

use super::{
    handlers::{
        bacnet::BacnetHandler, dnp3::Dnp3Handler, enip::EnipHandler, iec104::Iec104Handler,
        modbus::ModbusHandler, profinet::ProfinetDcpHandler, s7::S7Handler,
    },
    infer_device_type, AssetInfo, AssetSignatureMatch, ConnectionInfo, DeepParseInfo, LldpDetail,
    PacketSummary, SnmpDetail,
};
use super::protocol_handler::ProtocolHandler;


/// Processes packets through the full pipeline:
/// protocol identification → deep parse → connection tracking → topology building.
///
/// Protocol-specific accumulation is delegated to [`ProtocolHandler`] impls in
/// `handlers/`. To add a new protocol: implement `ProtocolHandler`, register the
/// handler in `PacketProcessor::new()`, and add a `DeepParseResult` arm if needed.
///
/// Used by both PCAP import and live capture.
pub struct PacketProcessor {
    pub topo_builder: TopologyBuilder,

    /// Per-protocol accumulators. Each handler is called for every packet.
    handlers: Vec<Box<dyn ProtocolHandler>>,

    // ── Core per-IP / per-connection accumulators ──────────────────────────
    connections: HashMap<String, ConnectionInfo>,
    packet_summaries: HashMap<String, Vec<PacketSummary>>,
    asset_protocols: HashMap<String, HashSet<IcsProtocol>>,
    asset_macs: HashMap<String, String>,
    asset_packet_counts: HashMap<String, u64>,
    asset_first_seen: HashMap<String, String>,
    asset_last_seen: HashMap<String, String>,
    server_ips: HashSet<String>,
    all_protocols: HashSet<String>,
    conn_origin_files: HashMap<String, HashSet<String>>,

    // ── Non-IP-layer protocol accumulators ────────────────────────────────
    /// LLDP info keyed by the sender MAC address (e.g. "aa:bb:cc:dd:ee:ff").
    /// Multiple LLDP frames from the same device are merged (last-write-wins).
    lldp_by_mac: HashMap<String, LldpInfo>,

    /// Redundancy protocol frames observed. Keyed by source MAC; last-write-wins
    /// within each MAC so we keep the most recent frame per sender.
    redundancy_by_mac: HashMap<String, RedundancyInfo>,

    /// SNMP device identity extracted from GET-Response packets.
    /// Keyed by the responding device's IP (src_ip when src_port == 161).
    snmp_device_info: HashMap<String, SnmpDeviceInfo>,

    // ── Signature matching ────────────────────────────────────────────────
    ip_packets: HashMap<String, Vec<PacketData>>,

    /// Communication pattern analyzer — collects timestamps per connection pair
    pattern_analyzer: PatternAnalyzer,

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
            topo_builder: TopologyBuilder::new(),
            handlers,
            connections: HashMap::new(),
            packet_summaries: HashMap::new(),
            asset_protocols: HashMap::new(),
            asset_macs: HashMap::new(),
            asset_packet_counts: HashMap::new(),
            asset_first_seen: HashMap::new(),
            asset_last_seen: HashMap::new(),
            server_ips: HashSet::new(),
            all_protocols: HashSet::new(),
            conn_origin_files: HashMap::new(),
            lldp_by_mac: HashMap::new(),
            redundancy_by_mac: HashMap::new(),
            snmp_device_info: HashMap::new(),
            ip_packets: HashMap::new(),
            pattern_analyzer: PatternAnalyzer::new(),
            total_packets: 0,
        }
    }

    /// Process a single packet through the pipeline.
    pub fn process_packet(&mut self, packet: &ParsedPacket) {
        // LLDP packets have a sentinel src_ip of "lldp:<mac>" — handle them
        // separately before the IP-based pipeline since they carry no IP header.
        if packet.src_ip.starts_with("lldp:") {
            if let Some(ref mac) = packet.src_mac {
                if let Some(lldp_info) = parse_lldp(&packet.payload) {
                    self.lldp_by_mac.insert(mac.clone(), lldp_info);
                }
            }
            return;
        }

        // Redundancy protocol packets (MRP/RSTP/HSR/PRP/DLR) use the sentinel
        // prefix "redundancy:<proto>" in src_ip. Parse and store by source MAC.
        if let Some(proto_hint) = packet.src_ip.strip_prefix("redundancy:") {
            if let Some(ref mac) = packet.src_mac {
                if let Some(info) = parse_redundancy(&packet.payload, proto_hint, mac) {
                    self.redundancy_by_mac.insert(mac.clone(), info);
                }
            }
            return;
        }

        let protocol = identify_protocol(packet);
        let proto_str = format!("{:?}", protocol);
        self.all_protocols.insert(proto_str.clone());
        self.total_packets += 1;

        let timestamp = packet.timestamp.to_rfc3339();

        // Track asset protocols
        self.asset_protocols
            .entry(packet.src_ip.clone())
            .or_default()
            .insert(protocol);
        self.asset_protocols
            .entry(packet.dst_ip.clone())
            .or_default()
            .insert(protocol);

        // Track MACs
        if let Some(ref mac) = packet.src_mac {
            self.asset_macs
                .entry(packet.src_ip.clone())
                .or_insert_with(|| mac.clone());
        }
        if let Some(ref mac) = packet.dst_mac {
            self.asset_macs
                .entry(packet.dst_ip.clone())
                .or_insert_with(|| mac.clone());
        }

        // Track packet counts
        *self
            .asset_packet_counts
            .entry(packet.src_ip.clone())
            .or_insert(0) += 1;
        *self
            .asset_packet_counts
            .entry(packet.dst_ip.clone())
            .or_insert(0) += 1;

        // Track timestamps
        self.asset_first_seen
            .entry(packet.src_ip.clone())
            .or_insert_with(|| timestamp.clone());
        self.asset_last_seen
            .insert(packet.src_ip.clone(), timestamp.clone());
        self.asset_first_seen
            .entry(packet.dst_ip.clone())
            .or_insert_with(|| timestamp.clone());
        self.asset_last_seen
            .insert(packet.dst_ip.clone(), timestamp.clone());

        // Detect servers using well-known OT service ports
        if is_ot_server_port(packet.dst_port) {
            self.server_ips.insert(packet.dst_ip.clone());
        }
        if is_ot_server_port(packet.src_port) {
            self.server_ips.insert(packet.src_ip.clone());
        }

        // Build connection key (directional: src→dst on protocol)
        let conn_key = format!(
            "{}:{}->{}:{}:{}",
            packet.src_ip, packet.src_port, packet.dst_ip, packet.dst_port, proto_str
        );

        let conn = self
            .connections
            .entry(conn_key.clone())
            .or_insert_with(|| ConnectionInfo {
                id: Uuid::new_v4().to_string(),
                src_ip: packet.src_ip.clone(),
                src_port: packet.src_port,
                src_mac: packet.src_mac.clone(),
                dst_ip: packet.dst_ip.clone(),
                dst_port: packet.dst_port,
                dst_mac: packet.dst_mac.clone(),
                protocol: proto_str.clone(),
                transport: format!("{:?}", packet.transport).to_lowercase(),
                packet_count: 0,
                byte_count: 0,
                first_seen: timestamp.clone(),
                last_seen: timestamp.clone(),
                origin_files: Vec::new(),
            });

        conn.packet_count += 1;
        conn.byte_count += packet.length as u64;
        conn.last_seen = timestamp.clone();

        // Track origin files per connection
        self.conn_origin_files
            .entry(conn_key.clone())
            .or_default()
            .insert(packet.origin_file.clone());

        // Store packet summary (cap at 1000 per connection)
        let summaries = self.packet_summaries.entry(conn.id.clone()).or_default();
        if summaries.len() < 1000 {
            summaries.push(PacketSummary {
                timestamp,
                src_ip: packet.src_ip.clone(),
                dst_ip: packet.dst_ip.clone(),
                src_port: packet.src_port,
                dst_port: packet.dst_port,
                protocol: proto_str.clone(),
                length: packet.length,
                origin_file: packet.origin_file.clone(),
            });
        }

        // ── Deep Protocol Parsing ────────────────────────────────
        // LLDP is handled by the early-return above; deep_parse() never returns
        // Lldp since it's not an IP-layer protocol.
        if let Some(deep_result) = deep_parse(packet, protocol) {
            if !matches!(deep_result, DeepParseResult::Lldp(_)) {
                for handler in &mut self.handlers {
                    handler.process(packet, &deep_result);
                }
            }
        }

        // SNMP GET-Response: extract device identity from responses (src port 161)
        if packet.src_port == 161 && !packet.payload.is_empty() {
            if let Some(dev_info) = parse_snmp_response(&packet.payload) {
                self.snmp_device_info
                    .insert(packet.src_ip.clone(), dev_info);
            }
        }

        // Feed into topology builder
        self.topo_builder.add_connection(
            &packet.src_ip,
            &packet.dst_ip,
            packet.src_mac.as_deref(),
            packet.dst_mac.as_deref(),
            protocol,
            packet.length as u64,
        );

        // Record packet for communication pattern analysis (O(1))
        let ts_epoch_pattern = packet.timestamp.timestamp() as f64
            + packet.timestamp.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
        self.pattern_analyzer.record_packet(
            &packet.src_ip,
            &packet.dst_ip,
            packet.dst_port,
            &proto_str,
            ts_epoch_pattern,
            packet.length as u64,
        );

        // Accumulate signature matching data (PacketData per IP)
        let pkt_data = PacketData {
            src_ip: packet.src_ip.clone(),
            dst_ip: packet.dst_ip.clone(),
            src_port: packet.src_port,
            dst_port: packet.dst_port,
            src_mac: packet.src_mac.clone(),
            dst_mac: packet.dst_mac.clone(),
            transport: format!("{:?}", packet.transport).to_lowercase(),
            protocol: format!("{:?}", protocol).to_lowercase(),
            payload: packet.payload.clone(),
            length: packet.length,
        };

        // Cap signature-matching packet storage at 200 per IP.
        // The signature engine only needs a small sample to identify a device;
        // storing all packets would consume gigabytes for large captures.
        const IP_PACKET_CAP: usize = 200;
        let src_entry = self.ip_packets.entry(packet.src_ip.clone()).or_default();
        if src_entry.len() < IP_PACKET_CAP {
            src_entry.push(pkt_data.clone());
        }
        let dst_entry = self.ip_packets.entry(packet.dst_ip.clone()).or_default();
        if dst_entry.len() < IP_PACKET_CAP {
            dst_entry.push(pkt_data);
        }
    }

    /// Build deep parse info by delegating to each registered handler, then
    /// appending LLDP and SNMP data (which are not routed through ProtocolHandler
    /// because they arrive on non-IP-layer frames).
    pub fn build_deep_parse_info(&self) -> HashMap<String, DeepParseInfo> {
        let mut deep_parse_info: HashMap<String, DeepParseInfo> = HashMap::new();

        // Each handler populates its protocol-specific field.
        for handler in &self.handlers {
            handler.finalize(&mut deep_parse_info);
        }

        // LLDP: match by MAC address (asset_macs maps IP → MAC).
        for (ip, mac) in &self.asset_macs {
            if let Some(lldp_info) = self.lldp_by_mac.get(mac) {
                let mgmt_addrs: Vec<String> = lldp_info
                    .management_addresses
                    .iter()
                    .map(|a| format!("{} ({})", a.address, a.addr_type))
                    .collect();
                let lldp_detail = LldpDetail {
                    system_name: lldp_info.system_name.clone(),
                    system_description: lldp_info.system_description.clone(),
                    chassis_id: lldp_info.chassis_id.clone(),
                    port_id: lldp_info.port_id.clone(),
                    capability_summary: lldp_info.capability_summary.clone(),
                    management_addresses: mgmt_addrs,
                    vlan_ids: lldp_info.vlan_ids.clone(),
                    vendor: lldp_info.vendor.clone(),
                    model: lldp_info.model.clone(),
                    firmware: lldp_info.firmware.clone(),
                };
                deep_parse_info.entry(ip.clone()).or_default().lldp = Some(lldp_detail);
            }
        }

        // SNMP: keyed directly by IP.
        for (ip, snmp_info) in &self.snmp_device_info {
            let snmp_detail = SnmpDetail {
                sys_descr: snmp_info.sys_descr.clone(),
                sys_name: snmp_info.sys_name.clone(),
                sys_location: snmp_info.sys_location.clone(),
                sys_object_id: snmp_info.sys_object_id.clone(),
                sys_uptime_cs: snmp_info.sys_uptime_cs,
                sys_contact: snmp_info.sys_contact.clone(),
                vendor: snmp_info.vendor.clone(),
            };
            deep_parse_info.entry(ip.clone()).or_default().snmp = Some(snmp_detail);
        }

        deep_parse_info
    }

    /// Collect all observed redundancy protocol frames as a flat list.
    ///
    /// Returns one `RedundancyInfo` per unique source MAC (last-frame-wins).
    pub fn build_redundancy_info(&self) -> Vec<RedundancyInfo> {
        self.redundancy_by_mac.values().cloned().collect()
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
        for (ip, packets) in &self.ip_packets {
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
        let all_ips: HashSet<String> = self.asset_protocols.keys().cloned().collect();
        let mut assets: Vec<AssetInfo> = Vec::new();

        for ip in &all_ips {
            let protocols: Vec<IcsProtocol> = self
                .asset_protocols
                .get(ip)
                .map(|s| s.iter().copied().collect())
                .unwrap_or_default();

            let is_server = self.server_ips.contains(ip);
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
            let mac = self.asset_macs.get(ip);
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
            if let Some(mac_addr) = self.asset_macs.get(ip) {
                if let Some(lldp) = self.lldp_by_mac.get(mac_addr) {
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
                mac_address: self.asset_macs.get(ip).cloned(),
                hostname,
                device_type,
                vendor,
                protocols: protocols
                    .iter()
                    .map(|p| format!("{:?}", p).to_lowercase())
                    .collect(),
                first_seen: self.asset_first_seen.get(ip).cloned().unwrap_or_default(),
                last_seen: self.asset_last_seen.get(ip).cloned().unwrap_or_default(),
                notes: String::new(),
                purdue_level: None,
                tags: Vec::new(),
                packet_count: *self.asset_packet_counts.get(ip).unwrap_or(&0),
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
        for (conn_key, conn) in &mut self.connections {
            if let Some(files) = self.conn_origin_files.get(conn_key) {
                conn.origin_files = files.iter().cloned().collect();
                conn.origin_files.sort();
            }
        }
        self.connections.values().cloned().collect()
    }

    /// Get a snapshot of packet summaries.
    pub fn get_packet_summaries(&self) -> HashMap<String, Vec<PacketSummary>> {
        self.packet_summaries.clone()
    }

    /// Get protocols detected so far.
    pub fn get_protocols_detected(&self) -> Vec<String> {
        self.all_protocols.iter().cloned().collect()
    }

    /// Compute per-connection-pair statistics and detect pattern anomalies.
    ///
    /// Returns `(stats, anomalies)` derived from Welford accumulators.
    /// Safe to call multiple times — no mutable state in PatternAnalyzer.
    pub fn build_pattern_results(&mut self) -> (Vec<ConnectionStats>, Vec<PatternAnomaly>) {
        let stats = self.pattern_analyzer.compute_stats();
        let anomalies = PatternAnalyzer::detect_anomalies(&stats);
        (stats, anomalies)
    }
}
