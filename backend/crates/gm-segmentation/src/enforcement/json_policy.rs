//! JSON Policy format generator.

use std::collections::HashMap;

use crate::{CommunicationMatrix, EnforcementConfig, EnforcementFormat, ZoneModel};

use super::vendor_context::vendor_remark_for_rule;

/// JSON Policy — structured `{metadata, zones, conduits, rules, default_action}`.
pub(super) fn gen_json_policy(
    matrix: &CommunicationMatrix,
    zone_model: &ZoneModel,
    zone_vendors: &HashMap<String, Vec<String>>,
) -> EnforcementConfig {
    let zones_json: Vec<serde_json::Value> = zone_model
        .zones
        .iter()
        .map(|z| {
            serde_json::json!({
                "id": z.id,
                "name": z.name,
                "purdue_levels": z.purdue_levels,
                "asset_count": z.asset_count,
                "security_level": format!("{:?}", z.security_level).to_lowercase()
            })
        })
        .collect();

    let conduits_json: Vec<serde_json::Value> = zone_model
        .conduits
        .iter()
        .map(|c| {
            serde_json::json!({
                "id": c.id,
                "src_zone_id": c.src_zone_id,
                "dst_zone_id": c.dst_zone_id,
                "cross_purdue_risk": c.cross_purdue_risk
            })
        })
        .collect();

    let rules_json: Vec<serde_json::Value> = matrix
        .zone_pairs
        .iter()
        .flat_map(|p| {
            p.rules.iter().map(move |r| {
                let vendor_context = vendor_remark_for_rule(
                    zone_vendors,
                    &p.src_zone_id,
                    &p.dst_zone_id,
                    r.dst_port,
                )
                .unwrap_or_default();

                serde_json::json!({
                    "src_zone_id": p.src_zone_id,
                    "dst_zone_id": p.dst_zone_id,
                    "protocol": r.protocol,
                    "dst_port": r.dst_port,
                    "risk": format!("{:?}", r.risk).to_lowercase(),
                    "packet_count": r.packet_count,
                    "justification": r.justification,
                    "vendor_context": vendor_context
                })
            })
        })
        .collect();

    let rule_count = rules_json.len();

    let policy = serde_json::json!({
        "metadata": {
            "generated_by": "Kusanagi Kajiki",
            "format": "json_policy",
            "coverage_percent": matrix.coverage_percent,
            "zone_score": zone_model.zone_score
        },
        "zones": zones_json,
        "conduits": conduits_json,
        "rules": rules_json,
        "default_action": matrix.default_action,
        "recommendations": zone_model.recommendations
    });

    let content = serde_json::to_string_pretty(&policy)
        .unwrap_or_else(|e| format!("{{\"error\": \"serialization failed: {e}\"}}"));

    EnforcementConfig::new(EnforcementFormat::JsonPolicy, content, rule_count)
}
