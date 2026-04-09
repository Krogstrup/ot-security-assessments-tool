//! Asset inventory synthesis from packet-processor runtime observations.

use std::collections::{HashMap, HashSet};

use gm_analysis::infer_device_type;
use gm_db::{GeoIpLookup, OuiLookup};
use gm_parsers::{DeepParseInfo, IcsProtocol, LldpInfo};
use gm_signatures::{PacketData, SignatureEngine};
use gm_types::{AssetInfo, AssetSignatureMatch};

/// Snapshot of runtime observations needed to build inventory assets.
pub struct AssetInventoryBuildInput<'a> {
    pub ip_packets: &'a HashMap<String, Vec<PacketData>>,
    pub asset_protocols: &'a HashMap<String, HashSet<IcsProtocol>>,
    pub server_ips: &'a HashSet<String>,
    pub asset_macs: &'a HashMap<String, String>,
    pub asset_packet_counts: &'a HashMap<String, u64>,
    pub asset_first_seen: &'a HashMap<String, String>,
    pub asset_last_seen: &'a HashMap<String, String>,
    pub lldp_by_mac: &'a HashMap<String, LldpInfo>,
}

/// Build final asset list with signature/OUI/deep-parse/LLDP enrichment.
pub fn build_assets_from_observations(
    input: &AssetInventoryBuildInput<'_>,
    engine: &SignatureEngine,
    deep_parse_info: &HashMap<String, DeepParseInfo>,
    oui_lookup: &OuiLookup,
    geoip_lookup: &GeoIpLookup,
) -> (Vec<AssetInfo>, HashMap<String, Vec<AssetSignatureMatch>>) {
    let mut sig_results: HashMap<String, Vec<AssetSignatureMatch>> = HashMap::new();
    for (ip, packets) in input.ip_packets {
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

    let mut assets: Vec<AssetInfo> = Vec::new();
    for ip in input.asset_protocols.keys() {
        let protocols: Vec<IcsProtocol> = input
            .asset_protocols
            .get(ip)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default();
        let is_server = input.server_ips.contains(ip);
        let mut device_type = infer_device_type(&protocols, is_server);

        let sig_matches = sig_results.get(ip).cloned().unwrap_or_default();
        let best_match = sig_matches.first();

        let mut confidence = best_match
            .map(|m| m.confidence)
            .unwrap_or_else(|| default_confidence(&protocols));
        let mut vendor = best_match.and_then(|m| m.vendor.clone());
        let mut product_family = best_match.and_then(|m| m.product_family.clone());

        let mac = input.asset_macs.get(ip);
        let oui_vendor = mac.and_then(|m| oui_lookup.lookup(m).map(|v| v.to_string()));

        if vendor.is_none() {
            if let Some(ref oui_v) = oui_vendor {
                vendor = Some(oui_v.clone());
                confidence = confidence.max(3);
            }
        }

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
                    .filter_map(|&part| part)
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

        let mut hostname: Option<String> = None;
        if let Some(mac_addr) = input.asset_macs.get(ip) {
            if let Some(lldp) = input.lldp_by_mac.get(mac_addr) {
                apply_lldp_enrichment(
                    lldp,
                    &mut hostname,
                    &mut vendor,
                    &mut product_family,
                    &mut confidence,
                    &mut device_type,
                );
            }
        }

        let is_public_ip = GeoIpLookup::is_public_ip(ip);
        let country = geoip_lookup.lookup_country(ip);
        assets.push(AssetInfo {
            id: ip.clone(),
            ip_address: ip.clone(),
            mac_address: input.asset_macs.get(ip).cloned(),
            hostname,
            device_type,
            vendor,
            protocols: protocols
                .iter()
                .map(|p| format!("{:?}", p).to_lowercase())
                .collect(),
            first_seen: input.asset_first_seen.get(ip).cloned().unwrap_or_default(),
            last_seen: input.asset_last_seen.get(ip).cloned().unwrap_or_default(),
            notes: String::new(),
            purdue_level: None,
            tags: Vec::new(),
            packet_count: *input.asset_packet_counts.get(ip).unwrap_or(&0),
            confidence,
            product_family,
            signature_matches: sig_matches,
            oui_vendor,
            country,
            is_public_ip,
        });
    }

    sort_assets_for_inventory(&mut assets);
    (assets, sig_results)
}

fn default_confidence(protocols: &[IcsProtocol]) -> u8 {
    if protocols.iter().any(|p| *p != IcsProtocol::Unknown) {
        1
    } else {
        0
    }
}

fn apply_lldp_enrichment(
    lldp: &LldpInfo,
    hostname: &mut Option<String>,
    vendor: &mut Option<String>,
    product_family: &mut Option<String>,
    confidence: &mut u8,
    device_type: &mut String,
) {
    if let Some(ref sn) = lldp.system_name {
        *hostname = Some(sn.clone());
    }
    if vendor.is_none() {
        if let Some(ref lv) = lldp.vendor {
            *vendor = Some(lv.clone());
            *confidence = (*confidence).max(4);
        }
    }
    if product_family.is_none() {
        if let Some(ref lm) = lldp.model {
            *product_family = Some(lm.clone());
        }
    }

    if let (Some(cap), Some(enabled)) = (lldp.capabilities, lldp.enabled_capabilities) {
        use gm_parsers::lldp::caps;
        let active = if enabled != 0 { enabled } else { cap };
        let is_bridge = active & caps::BRIDGE != 0;
        let is_router = active & caps::ROUTER != 0;
        if *device_type == "unknown" {
            if is_bridge && !is_router {
                *device_type = "switch".to_string();
            } else if is_router {
                *device_type = "router".to_string();
            }
        }
    }
}

fn sort_assets_for_inventory(assets: &mut [AssetInfo]) {
    assets.sort_by(|a, b| {
        let a_ot = a.device_type != "it_device" && a.device_type != "unknown";
        let b_ot = b.device_type != "it_device" && b.device_type != "unknown";
        b_ot.cmp(&a_ot).then(b.packet_count.cmp(&a.packet_count))
    });
}

#[cfg(test)]
mod tests {
    use super::sort_assets_for_inventory;
    use gm_types::AssetInfo;

    fn asset(id: &str, device_type: &str, packet_count: u64) -> AssetInfo {
        AssetInfo {
            id: id.to_string(),
            ip_address: id.to_string(),
            mac_address: None,
            hostname: None,
            device_type: device_type.to_string(),
            vendor: None,
            protocols: Vec::new(),
            first_seen: String::new(),
            last_seen: String::new(),
            notes: String::new(),
            purdue_level: None,
            tags: Vec::new(),
            packet_count,
            confidence: 0,
            product_family: None,
            signature_matches: Vec::new(),
            oui_vendor: None,
            country: None,
            is_public_ip: false,
        }
    }

    #[test]
    fn sort_prioritizes_ot_assets_then_packet_count() {
        let mut assets = vec![
            asset("10.0.0.1", "it_device", 10),
            asset("10.0.0.2", "plc", 5),
            asset("10.0.0.3", "unknown", 50),
            asset("10.0.0.4", "hmi", 20),
        ];
        sort_assets_for_inventory(&mut assets);
        assert_eq!(assets[0].ip_address, "10.0.0.4");
        assert_eq!(assets[1].ip_address, "10.0.0.2");
        assert_eq!(assets[2].ip_address, "10.0.0.3");
        assert_eq!(assets[3].ip_address, "10.0.0.1");
    }
}
