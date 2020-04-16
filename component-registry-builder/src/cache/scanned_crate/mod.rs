use semver::Version;
use crate::RegisteredComponent;

mod serde_helper;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScannedCrate {
    pub name: String,
    #[serde(serialize_with = "serde_helper::serialize_semver")]
    #[serde(deserialize_with = "serde_helper::deserialize_semver")]
    pub version: Option<Version>,
    pub components: Vec<RegisteredComponent>,
    pub source_path: String,
}