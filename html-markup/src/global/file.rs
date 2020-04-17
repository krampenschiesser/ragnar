
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct File {
    pub name: String,
    pub last_modified: u64,
    pub size: u64,
    pub mime_type: String,
    pub content: Vec<u8>
}