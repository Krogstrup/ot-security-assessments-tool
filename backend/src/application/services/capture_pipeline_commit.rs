use std::collections::HashMap;

use gm_analysis::{ConnectionStats, PatternAnomaly};
use gm_db::{GeoIpLookup, OuiLookup};
use gm_parsers::{DeepParseInfo, RedundancyInfo};
use gm_signatures::SignatureEngine;
use gm_topology::TopologyGraph;
use gm_types::{AssetInfo, AssetSignatureMatch, ConnectionInfo, PacketSummary};

pub struct CapturePipelineCommitResult {
    pub connection_count: usize,
    pub asset_count: usize,
    pub protocols_detected: Vec<String>,
}

pub struct CapturePipelineDependencies<'a> {
    pub signature_engine: &'a SignatureEngine,
    pub oui_lookup: &'a OuiLookup,
    pub geoip_lookup: &'a GeoIpLookup,
}

pub struct CapturePipelineStateUpdate {
    pub topology: TopologyGraph,
    pub connections: Vec<ConnectionInfo>,
    pub packet_summaries: HashMap<String, Vec<PacketSummary>>,
    pub redundancy_protocols: Vec<RedundancyInfo>,
    pub imported_files: Vec<String>,
    pub assets: Vec<AssetInfo>,
    pub deep_parse_info: HashMap<String, DeepParseInfo>,
    pub connection_stats: Vec<ConnectionStats>,
    pub pattern_anomalies: Vec<PatternAnomaly>,
}

pub trait CapturePipelineSource {
    fn build_deep_parse_info(&self) -> HashMap<String, DeepParseInfo>;
    fn build_assets(
        &self,
        signature_engine: &SignatureEngine,
        deep_parse_info: &HashMap<String, DeepParseInfo>,
        oui_lookup: &OuiLookup,
        geoip_lookup: &GeoIpLookup,
    ) -> (Vec<AssetInfo>, HashMap<String, Vec<AssetSignatureMatch>>);
    fn topology_snapshot(&self) -> TopologyGraph;
    fn get_connections(&mut self) -> Vec<ConnectionInfo>;
    fn get_packet_summaries(&self) -> HashMap<String, Vec<PacketSummary>>;
    fn build_pattern_results(&mut self) -> (Vec<ConnectionStats>, Vec<PatternAnomaly>);
    fn build_redundancy_info(&self) -> Vec<RedundancyInfo>;
    fn get_protocols_detected(&self) -> Vec<String>;
}

pub fn compute_capture_pipeline_state(
    source: &mut dyn CapturePipelineSource,
    deps: &CapturePipelineDependencies<'_>,
    existing_imported_files: &[String],
    newly_imported_files: &[String],
) -> (CapturePipelineStateUpdate, CapturePipelineCommitResult) {
    let deep_parse_info = source.build_deep_parse_info();
    let (assets, sig_results) = source.build_assets(
        deps.signature_engine,
        &deep_parse_info,
        deps.oui_lookup,
        deps.geoip_lookup,
    );

    let mut topology = source.topology_snapshot();
    enrich_topology_with_signatures(&mut topology, &sig_results);

    let connections = source.get_connections();
    let packet_summaries = source.get_packet_summaries();
    let (connection_stats, pattern_anomalies) = source.build_pattern_results();
    let redundancy_protocols = source.build_redundancy_info();
    let protocols_detected = source.get_protocols_detected();

    let mut imported_files = existing_imported_files.to_vec();
    if !newly_imported_files.is_empty() {
        imported_files.extend(newly_imported_files.iter().cloned());
        imported_files.sort();
        imported_files.dedup();
    }

    let asset_count = assets.len();
    let connection_count = connections.len();

    (
        CapturePipelineStateUpdate {
            topology,
            connections,
            packet_summaries,
            redundancy_protocols,
            imported_files,
            assets,
            deep_parse_info,
            connection_stats,
            pattern_anomalies,
        },
        CapturePipelineCommitResult {
            connection_count,
            asset_count,
            protocols_detected,
        },
    )
}

fn enrich_topology_with_signatures(
    topology: &mut TopologyGraph,
    sig_results: &HashMap<String, Vec<AssetSignatureMatch>>,
) {
    for node in &mut topology.nodes {
        if let Some(sig_matches) = sig_results.get(&node.ip_address) {
            if let Some(best) = sig_matches.first() {
                if let Some(ref vendor) = best.vendor {
                    node.vendor = Some(vendor.clone());
                }
                if let Some(ref device_type) = best.device_type {
                    if best.confidence >= 3 {
                        node.device_type = device_type.clone();
                    }
                }
            }
        }
    }
}
