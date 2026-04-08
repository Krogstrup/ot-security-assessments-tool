//! Topology query: capped graph for frontend visualization.

use std::collections::HashSet;

use gm_constants::{MAX_TOPOLOGY_EDGES, MAX_TOPOLOGY_NODES};
use gm_topology::TopologyGraph;

use crate::commands::{support::read_state, AppState};

/// Get the current network topology graph for visualization.
///
/// Nodes are capped at [`gm_constants::MAX_TOPOLOGY_NODES`] by packet_count
/// descending. Edges are filtered to retained nodes and capped at
/// [`gm_constants::MAX_TOPOLOGY_EDGES`]. For smaller datasets the full graph
/// is returned unchanged.
pub fn get_topology(state: &AppState) -> Result<TopologyGraph, String> {
    let capture = read_state(&state.capture, "capture")?;
    let topo = &capture.topology;

    if topo.nodes.len() <= MAX_TOPOLOGY_NODES && topo.edges.len() <= MAX_TOPOLOGY_EDGES {
        return Ok(topo.clone());
    }

    let mut nodes = topo.nodes.clone();
    nodes.sort_by(|a, b| b.packet_count.cmp(&a.packet_count));
    nodes.truncate(MAX_TOPOLOGY_NODES);

    let retained: HashSet<&str> = nodes.iter().map(|n| n.id.as_str()).collect();

    let mut edges: Vec<_> = topo
        .edges
        .iter()
        .filter(|e| retained.contains(e.source.as_str()) && retained.contains(e.target.as_str()))
        .cloned()
        .collect();
    edges.sort_by(|a, b| b.packet_count.cmp(&a.packet_count));
    edges.truncate(MAX_TOPOLOGY_EDGES);

    Ok(TopologyGraph { nodes, edges })
}
