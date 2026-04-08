use std::collections::HashMap;

use gm_topology::TopologyGraph;

use crate::commands::processor::PacketProcessor;
use crate::commands::{support::read_state, support::write_state, AppState, AssetSignatureMatch};

pub struct CapturePipelineCommitResult {
    pub connection_count: usize,
    pub asset_count: usize,
    pub protocols_detected: Vec<String>,
}

pub fn commit_capture_pipeline_state(
    state: &AppState,
    processor: &mut PacketProcessor,
    imported_files: &[String],
) -> Result<CapturePipelineCommitResult, String> {
    let deep_parse_info = processor.build_deep_parse_info();
    let (assets, sig_results) = {
        let sigs = read_state(&state.signatures, "signatures")?;
        let inv = read_state(&state.inventory, "inventory")?;
        processor.build_assets(
            &sigs.signature_engine,
            &deep_parse_info,
            &inv.oui_lookup,
            &inv.geoip_lookup,
        )
    };

    let mut topology = processor.topo_builder.snapshot();
    enrich_topology_with_signatures(&mut topology, &sig_results);

    let connections = processor.get_connections();
    let packet_summaries = processor.get_packet_summaries();
    let (connection_stats, pattern_anomalies) = processor.build_pattern_results();
    let redundancy_protocols = processor.build_redundancy_info();
    let protocols_detected = processor.get_protocols_detected();

    let asset_count = assets.len();
    let connection_count = connections.len();

    {
        let mut cap = write_state(&state.capture, "capture")?;
        cap.topology = topology;
        cap.connections = connections;
        cap.packet_summaries = packet_summaries;
        cap.redundancy_protocols = redundancy_protocols;
        if !imported_files.is_empty() {
            cap.imported_files.extend(imported_files.iter().cloned());
            cap.imported_files.sort();
            cap.imported_files.dedup();
        }
    }
    {
        let mut inv = write_state(&state.inventory, "inventory")?;
        inv.assets = assets;
        inv.deep_parse_info = deep_parse_info;
    }
    {
        let mut analysis = write_state(&state.analysis, "analysis")?;
        analysis.connection_stats = connection_stats;
        analysis.pattern_anomalies = pattern_anomalies;
    }

    Ok(CapturePipelineCommitResult {
        connection_count,
        asset_count,
        protocols_detected,
    })
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
