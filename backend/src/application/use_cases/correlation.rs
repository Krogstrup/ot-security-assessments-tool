//! Correlate imported IDS/SIEM alerts with inventory metadata.

use gm_ingest::StoredAlert;
use gm_types::AssetInfo;
use serde::Serialize;

/// An IDS/SIEM alert enriched with device inventory information.
#[derive(Debug, Clone, Serialize)]
pub struct CorrelatedAlert {
    pub timestamp: String,
    pub src_ip: String,
    pub src_port: u16,
    pub dst_ip: String,
    pub dst_port: u16,
    pub signature_id: u64,
    pub signature: String,
    pub category: String,
    /// 1 = high, 2 = medium, 3 = low
    pub severity: u8,
    pub source: String,
    pub src_hostname: Option<String>,
    pub src_device_type: Option<String>,
    pub src_purdue_level: Option<u8>,
    pub dst_hostname: Option<String>,
    pub dst_device_type: Option<String>,
    pub dst_purdue_level: Option<u8>,
}

pub fn correlate_alerts(alerts: &[StoredAlert], assets: &[AssetInfo]) -> Vec<CorrelatedAlert> {
    let mut correlated = alerts
        .iter()
        .map(|a| correlate_alert(a, assets))
        .collect::<Vec<_>>();
    sort_alerts(&mut correlated);
    correlated
}

pub fn correlate_alerts_for_ip(
    ip: &str,
    alerts: &[StoredAlert],
    assets: &[AssetInfo],
) -> Vec<CorrelatedAlert> {
    let mut correlated = alerts
        .iter()
        .filter(|a| a.src_ip == ip || a.dst_ip == ip)
        .map(|a| correlate_alert(a, assets))
        .collect::<Vec<_>>();
    sort_alerts(&mut correlated);
    correlated
}

fn sort_alerts(alerts: &mut [CorrelatedAlert]) {
    alerts.sort_by(|a, b| {
        a.severity
            .cmp(&b.severity)
            .then(b.timestamp.cmp(&a.timestamp))
    });
}

fn correlate_alert(alert: &StoredAlert, assets: &[AssetInfo]) -> CorrelatedAlert {
    let (src_hostname, src_device_type, src_purdue_level) = lookup_device(&alert.src_ip, assets);
    let (dst_hostname, dst_device_type, dst_purdue_level) = lookup_device(&alert.dst_ip, assets);

    CorrelatedAlert {
        timestamp: alert.timestamp.clone(),
        src_ip: alert.src_ip.clone(),
        src_port: alert.src_port,
        dst_ip: alert.dst_ip.clone(),
        dst_port: alert.dst_port,
        signature_id: alert.signature_id,
        signature: alert.signature.clone(),
        category: alert.category.clone(),
        severity: alert.severity,
        source: alert.source.clone(),
        src_hostname,
        src_device_type,
        src_purdue_level,
        dst_hostname,
        dst_device_type,
        dst_purdue_level,
    }
}

fn lookup_device(ip: &str, assets: &[AssetInfo]) -> (Option<String>, Option<String>, Option<u8>) {
    if ip.is_empty() {
        return (None, None, None);
    }
    match assets.iter().find(|a| a.ip_address == ip) {
        Some(asset) => (
            asset.hostname.clone(),
            Some(asset.device_type.clone()),
            asset.purdue_level,
        ),
        None => (None, None, None),
    }
}
