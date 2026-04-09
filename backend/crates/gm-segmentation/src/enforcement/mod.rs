//! Phase 15D — Enforcement Config Export.
//!
//! Generates ready-to-deploy enforcement configurations in five formats from
//! the communication matrix. Extends `allowlist.rs` firewall rule generation to
//! zone-aware multi-format output.
//!
//! | Format                | Use case                                   |
//! |-----------------------|--------------------------------------------|
//! | CiscoIosAcl           | Most common OT managed switch              |
//! | CiscoAsaAcl           | Dedicated OT firewall deployments          |
//! | GenericFirewallTable  | Vendor-neutral TSV import                  |
//! | SuricataRules         | IDS monitoring before hard enforcement     |
//! | JsonPolicy            | Automation / SOAR integration              |
//!
//! - Cisco ACL names: sanitized uppercase, max 64 characters.
//! - Suricata SIDs: 9000001+.
//! - Vendor-aware remarks when PolicyGroup names encode a vendor suffix.

mod cisco_asa;
mod cisco_ios;
mod generic_table;
mod json_policy;
mod suricata;
pub(super) mod vendor_context;

use std::collections::HashMap;

use crate::{
    CommunicationMatrix, EnforcementConfig, EnforcementFormat, PolicyGroup, ZoneModel,
};

use cisco_asa::gen_cisco_asa;
use cisco_ios::gen_cisco_ios;
use generic_table::gen_generic_table;
use json_policy::gen_json_policy;
use suricata::gen_suricata;
use vendor_context::build_zone_vendors;

// ── Public API ───────────────────────────────────────────────────────────────

/// Generate enforcement configurations in all five formats.
///
/// When `groups` is non-empty, vendor-specific port context remarks are added
/// to rules whose zone members include a known vendor (Siemens, Rockwell, etc.).
pub fn generate_enforcement_configs(
    matrix: &CommunicationMatrix,
    zone_model: &ZoneModel,
    groups: &[PolicyGroup],
) -> Vec<EnforcementConfig> {
    let zone_names: HashMap<String, String> = zone_model
        .zones
        .iter()
        .map(|z| (z.id.clone(), z.name.clone()))
        .collect();

    let zone_vendors = build_zone_vendors(&zone_model.zones, groups);

    vec![
        gen_cisco_ios(&matrix.zone_pairs, &zone_names, &zone_vendors),
        gen_cisco_asa(
            &matrix.zone_pairs,
            &zone_names,
            &zone_model.zones,
            &zone_vendors,
        ),
        gen_generic_table(&matrix.zone_pairs, &zone_names, &zone_vendors),
        gen_suricata(&matrix.zone_pairs, &zone_names, &zone_vendors),
        gen_json_policy(matrix, zone_model, &zone_vendors),
    ]
}

/// Generate an enforcement configuration in a single requested format.
///
/// Thin wrapper around [`generate_enforcement_configs`] using an empty zone
/// model so zone IDs serve as fallback names. Preserves the
/// `pub use enforcement::build_enforcement_config` re-export in `lib.rs`.
pub fn build_enforcement_config(
    matrix: &CommunicationMatrix,
    format: EnforcementFormat,
) -> EnforcementConfig {
    let empty_model = ZoneModel {
        zones: Vec::new(),
        conduits: Vec::new(),
        zone_score: 0.0,
        recommendations: Vec::new(),
    };
    generate_enforcement_configs(matrix, &empty_model, &[])
        .into_iter()
        .find(|c| c.format == format)
        .unwrap_or_else(|| EnforcementConfig::new(format, String::new(), 0))
}

// ── Public helpers ───────────────────────────────────────────────────────────

/// Sanitize a zone name for use in a Cisco ACL name.
///
/// Replaces non-alphanumeric characters with `_` and converts to uppercase.
/// The caller is responsible for truncating the result to the IOS 64-char limit.
pub fn sanitize_acl_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}

/// Derive the /24 network address and inverse wildcard mask for an IPv4 string.
///
/// Returns `None` if `ip` is not a valid dotted-quad.
///
/// ```
/// # use gm_segmentation::enforcement::ip_to_network_and_wildcard;
/// assert_eq!(
///     ip_to_network_and_wildcard("10.0.1.55"),
///     Some(("10.0.1.0".to_string(), "0.0.0.255".to_string()))
/// );
/// ```
pub fn ip_to_network_and_wildcard(ip: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return None;
    }
    for p in &parts {
        p.parse::<u8>().ok()?;
    }
    Some((
        format!("{}.{}.{}.0", parts[0], parts[1], parts[2]),
        "0.0.0.255".to_string(),
    ))
}

/// Return the /24 CIDR prefix for an IPv4 address string.
///
/// Returns `None` if `ip` is not a valid dotted-quad.
///
/// ```
/// # use gm_segmentation::enforcement::ip_to_cidr;
/// assert_eq!(ip_to_cidr("10.0.1.55"), Some("10.0.1.0/24".to_string()));
/// ```
pub fn ip_to_cidr(ip: &str) -> Option<String> {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return None;
    }
    for p in &parts {
        p.parse::<u8>().ok()?;
    }
    Some(format!("{}.{}.{}.0/24", parts[0], parts[1], parts[2]))
}

/// Determine whether a protocol / port combination uses TCP or UDP.
///
/// Falls back to `"tcp"` for unknown combinations.
pub fn protocol_to_transport(protocol: &str, port: Option<u16>) -> &'static str {
    const UDP_PORTS: &[u16] = &[47808, 34962, 34963, 34964, 2222, 161, 162, 69, 123, 514];
    if let Some(p) = port {
        if UDP_PORTS.contains(&p) {
            return "udp";
        }
    }
    match protocol.to_lowercase().as_str() {
        "bacnet" | "profinet_dcp" | "snmp" | "tftp" | "dns" | "ntp" | "syslog" | "ssdp" => "udp",
        _ => "tcp",
    }
}

// ── Private helpers ──────────────────────────────────────────────────────────

/// Look up a zone display name; fall back to the raw zone ID.
pub(super) fn zone_name<'a>(zone_names: &'a HashMap<String, String>, id: &'a str) -> &'a str {
    zone_names.get(id).map(|s| s.as_str()).unwrap_or(id)
}

/// Build a sanitized Cisco ACL name from two zone display names.
/// Format: `ACL-{SRC_28}-TO-{DST_28}`, total ≤ 64 characters.
pub(super) fn build_acl_name(src_name: &str, dst_name: &str) -> String {
    let src: String = sanitize_acl_name(src_name).chars().take(28).collect();
    let dst: String = sanitize_acl_name(dst_name).chars().take(28).collect();
    format!("ACL-{src}-TO-{dst}")
}

/// Strip characters that would break Suricata rule syntax (`"` and `;`).
pub(super) fn sanitize_suricata_msg(s: &str) -> String {
    s.chars()
        .filter(|&c| c != '"' && c != ';')
        .take(200)
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        CommunicationMatrix, PolicyRule, RuleRisk, SecurityLevel, Zone, ZoneModel, ZonePairPolicy,
    };

    // ── Test fixtures ─────────────────────────────────────────────────────────

    fn make_matrix(proto: &str, port: u16, risk: RuleRisk) -> CommunicationMatrix {
        CommunicationMatrix {
            zone_pairs: vec![ZonePairPolicy {
                src_zone_id: "z-ctrl".to_string(),
                dst_zone_id: "z-ent".to_string(),
                rules: vec![PolicyRule {
                    protocol: proto.to_string(),
                    dst_port: Some(port),
                    risk,
                    justification: format!("Observed 100 packets, {proto}/{port}"),
                    packet_count: 100,
                }],
            }],
            default_action: "deny".to_string(),
            coverage_percent: 100.0,
        }
    }

    fn make_model(ctrl_assets: usize, ent_assets: usize) -> ZoneModel {
        ZoneModel {
            zones: vec![
                Zone {
                    id: "z-ctrl".to_string(),
                    name: "Control Zone".to_string(),
                    purdue_levels: vec![0, 1],
                    policy_group_ids: Vec::new(),
                    security_level: SecurityLevel::Sl3,
                    asset_count: ctrl_assets,
                },
                Zone {
                    id: "z-ent".to_string(),
                    name: "Enterprise Zone".to_string(),
                    purdue_levels: vec![4],
                    policy_group_ids: Vec::new(),
                    security_level: SecurityLevel::Sl1,
                    asset_count: ent_assets,
                },
            ],
            conduits: Vec::new(),
            zone_score: 1.0,
            recommendations: Vec::new(),
        }
    }

    fn ios_config(matrix: &CommunicationMatrix, model: &ZoneModel) -> EnforcementConfig {
        generate_enforcement_configs(matrix, model, &[])
            .into_iter()
            .find(|c| c.format == EnforcementFormat::CiscoIosAcl)
            .unwrap()
    }

    fn asa_config(matrix: &CommunicationMatrix, model: &ZoneModel) -> EnforcementConfig {
        generate_enforcement_configs(matrix, model, &[])
            .into_iter()
            .find(|c| c.format == EnforcementFormat::CiscoAsaAcl)
            .unwrap()
    }

    fn table_config(matrix: &CommunicationMatrix, model: &ZoneModel) -> EnforcementConfig {
        generate_enforcement_configs(matrix, model, &[])
            .into_iter()
            .find(|c| c.format == EnforcementFormat::GenericFirewallTable)
            .unwrap()
    }

    fn suricata_config(matrix: &CommunicationMatrix, model: &ZoneModel) -> EnforcementConfig {
        generate_enforcement_configs(matrix, model, &[])
            .into_iter()
            .find(|c| c.format == EnforcementFormat::SuricataRules)
            .unwrap()
    }

    fn json_config(matrix: &CommunicationMatrix, model: &ZoneModel) -> EnforcementConfig {
        generate_enforcement_configs(matrix, model, &[])
            .into_iter()
            .find(|c| c.format == EnforcementFormat::JsonPolicy)
            .unwrap()
    }

    // ── test_cisco_ios_permit_syntax ──────────────────────────────────────────

    #[test]
    fn test_cisco_ios_permit_syntax() {
        let matrix = make_matrix("modbus", 502, RuleRisk::Low);
        let model = make_model(2, 2);
        let cfg = ios_config(&matrix, &model);
        assert!(
            cfg.content.contains("permit tcp any any eq 502"),
            "IOS ACL must include: permit tcp any any eq 502"
        );
        assert!(
            cfg.content.contains("ip access-list extended"),
            "IOS ACL must begin with: ip access-list extended"
        );
    }

    // ── test_cisco_ios_deny_default ───────────────────────────────────────────

    #[test]
    fn test_cisco_ios_deny_default() {
        let matrix = make_matrix("modbus", 502, RuleRisk::Low);
        let model = make_model(2, 2);
        let cfg = ios_config(&matrix, &model);
        assert!(
            cfg.content.contains("deny ip any any log"),
            "IOS ACL must include trailing: deny ip any any log"
        );
    }

    // ── test_cisco_ios_remark ─────────────────────────────────────────────────

    #[test]
    fn test_cisco_ios_remark() {
        let matrix = make_matrix("modbus", 502, RuleRisk::Low);
        let model = make_model(2, 2);
        let cfg = ios_config(&matrix, &model);
        assert!(
            cfg.content.contains("! Remark:"),
            "IOS ACL must include ! Remark: lines for zone pair and per-rule justification"
        );
        // Zone pair name in remark.
        assert!(cfg.content.contains("Control Zone"));
        assert!(cfg.content.contains("Enterprise Zone"));
    }

    // ── test_cisco_asa_object_groups ──────────────────────────────────────────

    #[test]
    fn test_cisco_asa_object_groups() {
        // Zone with >3 assets triggers object-group network generation.
        let matrix = make_matrix("modbus", 502, RuleRisk::Low);
        let model = make_model(5, 2); // ctrl has 5 assets → object-group
        let cfg = asa_config(&matrix, &model);
        assert!(
            cfg.content.contains("object-group network"),
            "ASA ACL must include object-group for zones with >3 assets"
        );
        assert!(
            cfg.content.contains("OBJ-CONTROL_ZONE"),
            "object-group name must be derived from the zone name"
        );
        // Enterprise zone has only 2 assets — should NOT generate an object-group.
        let obj_group_count = cfg.content.matches("object-group network OBJ-").count();
        assert_eq!(
            obj_group_count, 1,
            "only the zone with >3 assets should get an object-group"
        );
    }

    // ── test_generic_table_header ─────────────────────────────────────────────

    #[test]
    fn test_generic_table_header() {
        let matrix = CommunicationMatrix {
            zone_pairs: Vec::new(),
            default_action: "deny".to_string(),
            coverage_percent: 0.0,
        };
        let model = make_model(0, 0);
        let cfg = table_config(&matrix, &model);
        let first_line = cfg.content.lines().next().unwrap_or("");
        assert!(
            first_line.contains("Action"),
            "first line must be the TSV header"
        );
        assert!(
            first_line.contains("Src Zone"),
            "header must include Src Zone"
        );
        assert!(
            first_line.contains('\t'),
            "header columns must be tab-separated"
        );
        assert!(
            cfg.content.contains("DENY"),
            "table must always include the default deny row"
        );
    }

    // ── test_suricata_pass ────────────────────────────────────────────────────

    #[test]
    fn test_suricata_pass() {
        let matrix = make_matrix("modbus", 502, RuleRisk::Low);
        let model = make_model(2, 2);
        let cfg = suricata_config(&matrix, &model);
        assert!(
            cfg.content.contains("pass tcp"),
            "Suricata rules must include a pass tcp rule for modbus/502"
        );
        assert!(
            cfg.content.contains("502"),
            "Suricata pass rule must reference destination port 502"
        );
        assert!(
            cfg.content.contains("KNK-ALLOW"),
            "Suricata pass rule msg must contain KNK-ALLOW prefix"
        );
    }

    // ── test_suricata_sid_range ───────────────────────────────────────────────

    #[test]
    fn test_suricata_sid_range() {
        let matrix = make_matrix("modbus", 502, RuleRisk::Low);
        let model = make_model(2, 2);
        let cfg = suricata_config(&matrix, &model);
        assert!(
            cfg.content.contains("sid:9000001"),
            "first Suricata SID must be 9000001"
        );
        assert!(
            cfg.content.contains("drop ip"),
            "Suricata rules must end with a default drop ip rule"
        );
        assert!(
            cfg.content.contains("KNK-DENY"),
            "Suricata drop rule msg must contain KNK-DENY prefix"
        );
    }

    // ── test_json_parseable ───────────────────────────────────────────────────

    #[test]
    fn test_json_parseable() {
        let matrix = make_matrix("modbus", 502, RuleRisk::Low);
        let model = make_model(2, 2);
        let cfg = json_config(&matrix, &model);
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&cfg.content);
        assert!(
            parsed.is_ok(),
            "JSON policy output must be valid JSON: {:?}",
            parsed.err()
        );
        let val = parsed.unwrap();
        assert!(val.get("rules").is_some(), "JSON must have a 'rules' field");
        assert!(val.get("zones").is_some(), "JSON must have a 'zones' field");
        assert!(
            val.get("default_action").is_some(),
            "JSON must have a 'default_action' field"
        );
        assert!(
            val.get("metadata").is_some(),
            "JSON must have a 'metadata' field"
        );
        // Verify the rule is in there with the expected protocol.
        let rules = val["rules"].as_array().unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0]["protocol"], "modbus");
        // Vendor context field must be present (empty when no groups provided).
        assert!(
            rules[0].get("vendor_context").is_some(),
            "JSON rules must include vendor_context field"
        );
    }

    // ── test_all_five_formats ─────────────────────────────────────────────────

    #[test]
    fn test_all_five_formats() {
        let matrix = make_matrix("modbus", 502, RuleRisk::Low);
        let model = make_model(2, 2);
        let configs = generate_enforcement_configs(&matrix, &model, &[]);
        assert_eq!(
            configs.len(),
            5,
            "must generate exactly 5 enforcement configs"
        );
        // All five must be distinct formats.
        let mut seen_formats = std::collections::HashSet::new();
        for cfg in &configs {
            let fmt = format!("{:?}", cfg.format);
            assert!(
                seen_formats.insert(fmt.clone()),
                "duplicate format in output: {fmt}"
            );
        }
        assert_eq!(seen_formats.len(), 5, "all five formats must be distinct");
    }

    // ── test_rule_count_matches ───────────────────────────────────────────────

    #[test]
    fn test_rule_count_matches() {
        let mut matrix = make_matrix("modbus", 502, RuleRisk::Low);
        // Add a second rule to the same pair.
        matrix.zone_pairs[0].rules.push(PolicyRule {
            protocol: "http".to_string(),
            dst_port: Some(80),
            risk: RuleRisk::Low,
            justification: "Observed 50 packets".to_string(),
            packet_count: 50,
        });
        let model = make_model(2, 2);
        let cfg = ios_config(&matrix, &model);
        assert_eq!(
            cfg.rule_count, 2,
            "rule_count must match number of permit rules"
        );
    }

    // ── test_sanitize_acl_name ────────────────────────────────────────────────

    #[test]
    fn test_sanitize_acl_name() {
        assert_eq!(sanitize_acl_name("Control Zone"), "CONTROL_ZONE");
        assert_eq!(sanitize_acl_name("L1-Modbus"), "L1_MODBUS");
        assert_eq!(sanitize_acl_name("Enterprise IT"), "ENTERPRISE_IT");
        // Must not include spaces or special characters.
        let result = sanitize_acl_name("Test Zone (SL3)");
        assert!(!result.contains(' '));
        assert!(!result.contains('('));
    }

    // ── test_ip_helpers ───────────────────────────────────────────────────────

    #[test]
    fn test_ip_helpers() {
        // ip_to_network_and_wildcard
        assert_eq!(
            ip_to_network_and_wildcard("10.0.1.55"),
            Some(("10.0.1.0".to_string(), "0.0.0.255".to_string()))
        );
        assert_eq!(ip_to_network_and_wildcard("not-an-ip"), None);
        assert_eq!(ip_to_network_and_wildcard("1.2.3"), None);

        // ip_to_cidr
        assert_eq!(
            ip_to_cidr("192.168.100.200"),
            Some("192.168.100.0/24".to_string())
        );
        assert_eq!(ip_to_cidr("bad"), None);

        // protocol_to_transport
        assert_eq!(protocol_to_transport("modbus", Some(502)), "tcp");
        assert_eq!(protocol_to_transport("bacnet", Some(47808)), "udp");
        assert_eq!(protocol_to_transport("snmp", Some(161)), "udp");
        assert_eq!(protocol_to_transport("unknown_proto", None), "tcp");
    }

    // ── test_vendor_remark_in_cisco_ios ──────────────────────────────────────

    #[test]
    fn test_vendor_remark_in_cisco_ios() {
        use crate::{Criticality, DeviceCategory, PolicyGroup};

        let matrix = make_matrix("s7comm", 102, RuleRisk::Low);

        // Create a group with vendor suffix "Siemens" and attach it to z-ctrl.
        let group = PolicyGroup::new(
            "L1-S7-Siemens",
            vec!["10.0.0.1".to_string()],
            Some(1),
            DeviceCategory::Plc,
            SecurityLevel::Sl3,
            Criticality::High,
        );

        let model = ZoneModel {
            zones: vec![
                Zone {
                    id: "z-ctrl".to_string(),
                    name: "Control Zone".to_string(),
                    purdue_levels: vec![0, 1],
                    policy_group_ids: vec![group.id.clone()],
                    security_level: SecurityLevel::Sl3,
                    asset_count: 2,
                },
                Zone {
                    id: "z-ent".to_string(),
                    name: "Enterprise Zone".to_string(),
                    purdue_levels: vec![4],
                    policy_group_ids: Vec::new(),
                    security_level: SecurityLevel::Sl1,
                    asset_count: 2,
                },
            ],
            conduits: Vec::new(),
            zone_score: 1.0,
            recommendations: Vec::new(),
        };

        let configs = generate_enforcement_configs(&matrix, &model, &[group]);
        let ios = configs
            .iter()
            .find(|c| c.format == EnforcementFormat::CiscoIosAcl)
            .unwrap();

        assert!(
            ios.content.contains("Siemens"),
            "IOS ACL must include Siemens vendor remark when group name encodes vendor.\nGot:\n{}",
            ios.content
        );
        assert!(
            ios.content.contains("S7comm PLC communication"),
            "IOS ACL must include vendor port context for Siemens/102"
        );
    }

    // ── test_vendor_port_context ─────────────────────────────────────────────

    #[test]
    fn test_vendor_port_context() {
        use super::vendor_context::vendor_port_context;

        // Known combinations.
        assert_eq!(
            vendor_port_context("Siemens", 102),
            Some("S7comm PLC communication")
        );
        assert_eq!(
            vendor_port_context("Siemens AG", 443),
            Some("SCALANCE web management")
        );
        assert_eq!(
            vendor_port_context("Rockwell Automation", 44818),
            Some("EtherNet/IP explicit messaging")
        );
        assert_eq!(
            vendor_port_context("Schneider Electric", 502),
            Some("Modbus TCP")
        );
        assert_eq!(vendor_port_context("ABB Ltd", 502), Some("Modbus TCP"));
        assert_eq!(vendor_port_context("Honeywell", 502), Some("Modbus TCP"));
        // Unknown combination.
        assert_eq!(vendor_port_context("Siemens", 80), None);
        assert_eq!(vendor_port_context("UnknownVendor", 502), None);
    }
}
