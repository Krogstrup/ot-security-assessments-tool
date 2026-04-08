use crate::commands;
use crate::commands::data::{AssetSortBy, ConnectionSortBy, ProtocolStatsSortBy};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ImportPcapRequest {
    pub(crate) paths: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateProjectRequest {
    pub(crate) name: String,
    pub(crate) client_name: Option<String>,
    pub(crate) site_name: Option<String>,
    pub(crate) assessor_name: Option<String>,
    pub(crate) engagement_start: Option<String>,
    pub(crate) engagement_end: Option<String>,
    pub(crate) notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateProjectRequest {
    pub(crate) name: String,
    pub(crate) client_name: Option<String>,
    pub(crate) site_name: Option<String>,
    pub(crate) assessor_name: Option<String>,
    pub(crate) engagement_start: Option<String>,
    pub(crate) engagement_end: Option<String>,
    pub(crate) notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SetActiveProjectRequest {
    pub(crate) id: i64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AssetPagingQuery {
    pub(crate) page: Option<usize>,
    #[serde(alias = "pageSize")]
    pub(crate) page_size: Option<usize>,
    #[serde(alias = "sortBy")]
    pub(crate) sort_by: Option<AssetSortBy>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ConnectionPagingQuery {
    pub(crate) page: Option<usize>,
    #[serde(alias = "pageSize")]
    pub(crate) page_size: Option<usize>,
    #[serde(alias = "sortBy")]
    pub(crate) sort_by: Option<ConnectionSortBy>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ProtocolStatsQuery {
    #[serde(alias = "sortBy")]
    pub(crate) sort_by: Option<ProtocolStatsSortBy>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SaveSessionRequest {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExportSessionRequest {
    pub(crate) output_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImportSessionRequest {
    pub(crate) archive_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CompareSessionsRequest {
    pub(crate) baseline_session_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StartCaptureRequest {
    pub(crate) interface_name: String,
    pub(crate) bpf_filter: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StopCaptureRequest {
    pub(crate) save_path: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateAssetRequest {
    pub(crate) updates: commands::session::AssetUpdate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BulkUpdateAssetsRequest {
    pub(crate) asset_ids: Vec<String>,
    pub(crate) updates: commands::session::AssetUpdate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImportPathRequest {
    pub(crate) path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImportZeekRequest {
    pub(crate) paths: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImportSwitchPathRequest {
    pub(crate) path: String,
    pub(crate) switch_hostname: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TestSignatureRequest {
    pub(crate) yaml: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SaveSettingsRequest {
    pub(crate) settings: commands::system::UserSettings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OutputPathRequest {
    pub(crate) output_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SaveFramesCsvRequest {
    pub(crate) output_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GeneratePdfReportRequest {
    pub(crate) config: commands::export::ReportConfigInput,
    pub(crate) output_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExportSbomRequest {
    pub(crate) format: String,
    pub(crate) output_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SaveTopologyImageRequest {
    pub(crate) image_data: String,
    pub(crate) output_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExportFilteredPcapRequest {
    pub(crate) filter_ips: Vec<String>,
    pub(crate) filter_ports: Vec<u16>,
    pub(crate) output_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExportEnforcementConfigRequest {
    pub(crate) format: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConnectionIdRequest {
    pub(crate) connection_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct IpAddressRequest {
    pub(crate) ip_address: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ComplianceQuery {
    pub(crate) framework: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CveQuery {
    pub(crate) ip: String,
}
