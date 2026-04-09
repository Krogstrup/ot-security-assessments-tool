//! Protocol handler trait and processor core data structures.
//!
//! `PacketProcessor` is intentionally a coordinator over two pieces:
//! - `ProcessorCore`: shared cross-protocol state and packet bookkeeping.
//! - `Vec<Box<dyn ProtocolHandler>>`: protocol-specific accumulators.

use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use gm_analysis::{ConnectionStats, PatternAnalyzer, PatternAnomaly};
use gm_capture::ParsedPacket;
use gm_parsers::{
    deep_parse, identify_protocol, parse_lldp, parse_redundancy, parse_snmp_response,
    DeepParseResult, IcsProtocol, LldpInfo, RedundancyInfo, SnmpDeviceInfo,
};
use gm_signatures::PacketData;
use gm_topology::TopologyBuilder;
use gm_types::is_ot_server_port;

use super::{ConnectionInfo, DeepParseInfo, LldpDetail, PacketSummary, SnmpDetail};

/// Mutable state shared across protocols while packets are processed.
#[derive(Default)]
pub struct ProcessorCore {
    // ── Core per-IP / per-connection accumulators ──────────────────────────
    pub(crate) connections: HashMap<String, ConnectionInfo>,
    pub(crate) packet_summaries: HashMap<String, Vec<PacketSummary>>,
    pub(crate) asset_protocols: HashMap<String, HashSet<IcsProtocol>>,
    pub(crate) asset_macs: HashMap<String, String>,
    pub(crate) asset_packet_counts: HashMap<String, u64>,
    pub(crate) asset_first_seen: HashMap<String, String>,
    pub(crate) asset_last_seen: HashMap<String, String>,
    pub(crate) server_ips: HashSet<String>,
    pub(crate) all_protocols: HashSet<String>,
    pub(crate) conn_origin_files: HashMap<String, HashSet<String>>,

    // ── Non-IP-layer protocol accumulators ────────────────────────────────
    pub(crate) lldp_by_mac: HashMap<String, LldpInfo>,
    pub(crate) redundancy_by_mac: HashMap<String, RedundancyInfo>,
    pub(crate) snmp_device_info: HashMap<String, SnmpDeviceInfo>,

    // ── Signature matching ────────────────────────────────────────────────
    pub(crate) ip_packets: HashMap<String, Vec<PacketData>>,

    /// Communication pattern analyzer — collects timestamps per connection pair.
    pub(crate) pattern_analyzer: PatternAnalyzer,
}

impl ProcessorCore {
    /// Update shared packet-processing state. Returns deep-parse results for
    /// protocol handlers when available.
    pub fn process(
        &mut self,
        packet: &ParsedPacket,
        topo_builder: &mut TopologyBuilder,
        total_packets: &mut u64,
    ) -> Option<DeepParseResult> {
        // LLDP frames are handled before the IP pipeline.
        if packet.src_ip.starts_with("lldp:") {
            if let Some(ref mac) = packet.src_mac {
                if let Some(lldp_info) = parse_lldp(&packet.payload) {
                    self.lldp_by_mac.insert(mac.clone(), lldp_info);
                }
            }
            return None;
        }

        // Redundancy protocol frames (MRP/RSTP/HSR/PRP/DLR) use a sentinel src_ip.
        if let Some(proto_hint) = packet.src_ip.strip_prefix("redundancy:") {
            if let Some(ref mac) = packet.src_mac {
                if let Some(info) = parse_redundancy(&packet.payload, proto_hint, mac) {
                    self.redundancy_by_mac.insert(mac.clone(), info);
                }
            }
            return None;
        }

        let protocol = identify_protocol(packet);
        let proto_str = format!("{:?}", protocol);
        self.all_protocols.insert(proto_str.clone());
        *total_packets += 1;

        let timestamp = packet.timestamp.to_rfc3339();

        // Track protocols per source/destination IP.
        self.asset_protocols
            .entry(packet.src_ip.clone())
            .or_default()
            .insert(protocol);
        self.asset_protocols
            .entry(packet.dst_ip.clone())
            .or_default()
            .insert(protocol);

        // Track first observed MAC address for each IP.
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

        // Track packet counts.
        *self
            .asset_packet_counts
            .entry(packet.src_ip.clone())
            .or_insert(0) += 1;
        *self
            .asset_packet_counts
            .entry(packet.dst_ip.clone())
            .or_insert(0) += 1;

        // Track first/last seen timestamps.
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

        // Detect servers using well-known OT service ports.
        if is_ot_server_port(packet.dst_port) {
            self.server_ips.insert(packet.dst_ip.clone());
        }
        if is_ot_server_port(packet.src_port) {
            self.server_ips.insert(packet.src_ip.clone());
        }

        // Build a directional connection key (src→dst on protocol).
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

        // Track origin files per connection.
        self.conn_origin_files
            .entry(conn_key.clone())
            .or_default()
            .insert(packet.origin_file.clone());

        // Store packet summary (cap at 1000 per connection).
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

        // SNMP GET-Response: extract device identity from responses (src port 161).
        if packet.src_port == 161 && !packet.payload.is_empty() {
            if let Some(dev_info) = parse_snmp_response(&packet.payload) {
                self.snmp_device_info
                    .insert(packet.src_ip.clone(), dev_info);
            }
        }

        // Feed into topology builder.
        topo_builder.add_connection(
            &packet.src_ip,
            &packet.dst_ip,
            packet.src_mac.as_deref(),
            packet.dst_mac.as_deref(),
            protocol,
            packet.length as u64,
        );

        // Record packet for communication pattern analysis.
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

        // Accumulate signature matching data (PacketData per IP).
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
        const IP_PACKET_CAP: usize = 200;
        let src_entry = self.ip_packets.entry(packet.src_ip.clone()).or_default();
        if src_entry.len() < IP_PACKET_CAP {
            src_entry.push(pkt_data.clone());
        }
        let dst_entry = self.ip_packets.entry(packet.dst_ip.clone()).or_default();
        if dst_entry.len() < IP_PACKET_CAP {
            dst_entry.push(pkt_data);
        }

        deep_parse(packet, protocol).filter(|r| !matches!(r, DeepParseResult::Lldp(_)))
    }

    /// LLDP + SNMP details are accumulated in the shared core, not by
    /// protocol-specific handlers.
    pub fn populate_core_deep_parse(&self, deep_parse: &mut HashMap<String, DeepParseInfo>) {
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
                deep_parse.entry(ip.clone()).or_default().lldp = Some(lldp_detail);
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
            deep_parse.entry(ip.clone()).or_default().snmp = Some(snmp_detail);
        }
    }

    pub fn build_redundancy_info(&self) -> Vec<RedundancyInfo> {
        self.redundancy_by_mac.values().cloned().collect()
    }

    pub fn finalize_connections(&mut self) -> Vec<ConnectionInfo> {
        for (conn_key, conn) in &mut self.connections {
            if let Some(files) = self.conn_origin_files.get(conn_key) {
                conn.origin_files = files.iter().cloned().collect();
                conn.origin_files.sort();
            }
        }
        self.connections.values().cloned().collect()
    }

    pub fn protocols_detected(&self) -> Vec<String> {
        self.all_protocols.iter().cloned().collect()
    }

    pub fn pattern_results(&mut self) -> (Vec<ConnectionStats>, Vec<PatternAnomaly>) {
        let stats = self.pattern_analyzer.compute_stats();
        let anomalies = PatternAnalyzer::detect_anomalies(&stats);
        (stats, anomalies)
    }
}

/// Structured output assembled by protocol handlers.
#[derive(Default)]
pub struct ProcessorOutput {
    pub deep_parse: HashMap<String, DeepParseInfo>,
}

/// Extension point for per-protocol packet accumulation.
pub trait ProtocolHandler: Send {
    /// Inspect packet + deep-parse result and update protocol-local accumulators.
    fn process(&mut self, packet: &ParsedPacket, result: &DeepParseResult);

    /// Flush protocol-local accumulators into shared processor output.
    fn finalize(&self, output: &mut ProcessorOutput);
}
