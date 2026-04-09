//! Shared packet processing pipeline.
//!
//! `PacketProcessor` encapsulates the full processing pipeline used by both
//! PCAP import and live capture: protocol identification → deep parse →
//! connection tracking → topology building → signature matching.

use std::collections::HashMap;

use gm_analysis::{ConnectionStats, PatternAnomaly};
use gm_capture::ParsedPacket;
use gm_db::{GeoIpLookup, OuiLookup};
use gm_parsers::RedundancyInfo;
use gm_signatures::SignatureEngine;
use gm_topology::TopologyBuilder;

use crate::application::services::asset_inventory::{
    build_assets_from_observations, AssetInventoryBuildInput,
};

use super::protocol_handler::{ProcessorCore, ProcessorOutput, ProtocolHandler};
use super::{
    handlers::{
        bacnet::BacnetHandler, dnp3::Dnp3Handler, enip::EnipHandler, iec104::Iec104Handler,
        modbus::ModbusHandler, profinet::ProfinetDcpHandler, s7::S7Handler,
    },
    AssetInfo, AssetSignatureMatch, ConnectionInfo, DeepParseInfo, PacketSummary,
};

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
        if let Some(deep_result) =
            self.core
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
        let input = AssetInventoryBuildInput {
            ip_packets: &self.core.ip_packets,
            asset_protocols: &self.core.asset_protocols,
            server_ips: &self.core.server_ips,
            asset_macs: &self.core.asset_macs,
            asset_packet_counts: &self.core.asset_packet_counts,
            asset_first_seen: &self.core.asset_first_seen,
            asset_last_seen: &self.core.asset_last_seen,
            lldp_by_mac: &self.core.lldp_by_mac,
        };
        build_assets_from_observations(&input, engine, deep_parse_info, oui_lookup, geoip_lookup)
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
