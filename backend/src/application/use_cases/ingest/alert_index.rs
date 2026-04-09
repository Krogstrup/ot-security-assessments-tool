//! Alert storage helpers and Zeek per-device event index builder.

use std::collections::HashMap;

use gm_ingest::{DeviceZeekEvents, IngestedAlert, StoredAlert, ZeekEventSummary};
use gm_types::ConnectionInfo;

/// Convert an [`IngestedAlert`] to the persisted [`StoredAlert`] representation.
pub(super) fn ingested_alert_to_stored(alert: &IngestedAlert) -> StoredAlert {
    StoredAlert {
        timestamp: alert.timestamp.to_rfc3339(),
        src_ip: alert.src_ip.clone(),
        src_port: alert.src_port,
        dst_ip: alert.dst_ip.clone(),
        dst_port: alert.dst_port,
        signature_id: alert.signature_id,
        signature: alert.signature.clone(),
        category: alert.category.clone(),
        severity: alert.severity,
        source: alert.source.display_name().to_string(),
    }
}

/// Rebuild per-device Zeek event summaries from all connections tagged `[Zeek]`.
///
/// Called after each Zeek import to refresh the event index.
pub(super) fn rebuild_zeek_device_events(
    connections: &[ConnectionInfo],
    imported_alerts: &[StoredAlert],
) -> HashMap<String, DeviceZeekEvents> {
    let mut map: HashMap<String, (DeviceZeekEvents, std::collections::HashSet<String>)> =
        HashMap::new();

    for conn in connections {
        if !conn.origin_files.iter().any(|f| f.contains("Zeek")) {
            continue;
        }

        let log_type = classify_zeek_log_type(&conn.protocol, conn.dst_port);
        let timestamp = conn.first_seen.clone();

        for (device_ip, peer_ip) in [
            (conn.src_ip.clone(), conn.dst_ip.clone()),
            (conn.dst_ip.clone(), conn.src_ip.clone()),
        ] {
            let (events, peers) = map.entry(device_ip.clone()).or_insert_with(|| {
                (
                    DeviceZeekEvents {
                        device_ip: device_ip.clone(),
                        ..Default::default()
                    },
                    std::collections::HashSet::new(),
                )
            });

            match log_type.as_str() {
                "modbus" => events.modbus_events += 1,
                "dnp3" => events.dnp3_events += 1,
                "dns" => events.dns_queries += 1,
                "http" => events.http_requests += 1,
                _ => events.conn_log_entries += 1,
            }

            peers.insert(peer_ip.clone());

            if events.sample_events.len() < 50 {
                events.sample_events.push(ZeekEventSummary {
                    timestamp: timestamp.clone(),
                    log_type: log_type.clone(),
                    peer_ip: peer_ip.clone(),
                    summary: format!(
                        "{} {}:{} → {}:{} ({} pkts)",
                        conn.transport.to_uppercase(),
                        conn.src_ip,
                        conn.src_port,
                        conn.dst_ip,
                        conn.dst_port,
                        conn.packet_count
                    ),
                });
            }
        }
    }

    let alert_map: HashMap<String, u32> = {
        let mut m: HashMap<String, u32> = HashMap::new();
        for alert in imported_alerts {
            *m.entry(alert.src_ip.clone()).or_insert(0) += 1;
            *m.entry(alert.dst_ip.clone()).or_insert(0) += 1;
        }
        m
    };

    map.into_iter()
        .map(|(ip, (mut events, peers))| {
            events.unique_peers = peers.len() as u32;
            events.alert_count = alert_map.get(&ip).copied().unwrap_or(0);
            (ip, events)
        })
        .collect()
}

/// Map a connection protocol string + port to a Zeek log type label.
fn classify_zeek_log_type(protocol: &str, dst_port: u16) -> String {
    match protocol.to_lowercase().as_str() {
        "modbus" => "modbus".to_string(),
        "dnp3" => "dnp3".to_string(),
        "s7comm" => "s7comm".to_string(),
        _ => match dst_port {
            53 => "dns".to_string(),
            80 | 443 | 8080 | 8443 => "http".to_string(),
            _ => "conn".to_string(),
        },
    }
}
