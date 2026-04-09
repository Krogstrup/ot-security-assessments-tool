use std::path::PathBuf;

/// Resolved paths for bundled runtime resources (signatures, OUI, GeoIP).
///
/// Web/API builds resolve paths from environment variables plus local fallbacks.
pub struct ResourcePaths {
    pub signatures_dir: PathBuf,
    pub oui_path: PathBuf,
    pub geoip_path: PathBuf,
}

impl ResourcePaths {
    /// Resolve for headless/web builds: env vars, then CWD-relative fallbacks.
    ///
    /// Environment variables:
    /// - `KK_SIGNATURES_DIR` — override signatures directory
    /// - `KK_DATA_DIR` — override data directory (parent of `oui.tsv` and the MMDB)
    pub fn from_env() -> Self {
        let signatures_dir = std::env::var("KK_SIGNATURES_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| Self::probe_signatures_dir());
        let data_dir = std::env::var("KK_DATA_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| Self::probe_data_dir());
        Self {
            signatures_dir,
            oui_path: data_dir.join("oui.tsv"),
            geoip_path: data_dir.join("dbip-country-lite.mmdb"),
        }
    }

    fn probe_signatures_dir() -> PathBuf {
        [
            "signatures",
            "../backend/signatures",
            "backend/signatures",
        ]
        .iter()
        .map(PathBuf::from)
        .find(|p| p.exists())
        .unwrap_or_else(|| PathBuf::from("signatures"))
    }

    fn probe_data_dir() -> PathBuf {
        ["data", "../backend/data", "backend/data"]
            .iter()
            .map(PathBuf::from)
            .find(|p| p.join("oui.tsv").exists())
            .unwrap_or_else(|| PathBuf::from("data"))
    }
}
