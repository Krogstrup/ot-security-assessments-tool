//! Generic Firewall Table format generator (tab-separated).

use std::collections::HashMap;

use crate::{EnforcementConfig, EnforcementFormat, ZonePairPolicy};

use super::zone_name;
use super::vendor_context::vendor_remark_for_rule;
use super::protocol_to_transport;

/// Generic Firewall Table — tab-separated, one rule per row.
///
/// Columns: Action | Src Zone | Src Net | Dst Zone | Dst Net | Proto | Port | Dir | Risk | Justification
pub(super) fn gen_generic_table(
    pairs: &[ZonePairPolicy],
    zone_names: &HashMap<String, String>,
    zone_vendors: &HashMap<String, Vec<String>>,
) -> EnforcementConfig {
    let mut out = String::new();
    out.push_str(
        "Action\tSrc Zone\tSrc Net\tDst Zone\tDst Net\tProto\tPort\tDir\tRisk\tJustification\n",
    );

    let mut rule_count = 0usize;

    for pair in pairs {
        let src_name = zone_name(zone_names, &pair.src_zone_id);
        let dst_name = zone_name(zone_names, &pair.dst_zone_id);

        for rule in &pair.rules {
            let transport = protocol_to_transport(&rule.protocol, rule.dst_port);
            let port_str = rule
                .dst_port
                .map(|p| p.to_string())
                .unwrap_or_else(|| "any".to_string());
            let risk_str = format!("{:?}", rule.risk).to_lowercase();
            // Strip tabs to preserve TSV structure.
            let mut just = rule.justification.replace('\t', " ");

            // Append vendor context to justification.
            if let Some(vendor_ctx) = vendor_remark_for_rule(
                zone_vendors,
                &pair.src_zone_id,
                &pair.dst_zone_id,
                rule.dst_port,
            ) {
                just = format!("{just} [{vendor_ctx}]");
            }

            out.push_str(&format!(
                "ALLOW\t{src_name}\tany\t{dst_name}\tany\t{transport}\t{port_str}\t→\t{risk_str}\t{just}\n"
            ));
            rule_count += 1;
        }
    }

    // Default deny row.
    out.push_str("DENY\t*\t*\t*\t*\t*\t*\t*\t—\tDefault deny\n");

    EnforcementConfig::new(EnforcementFormat::GenericFirewallTable, out, rule_count)
}
