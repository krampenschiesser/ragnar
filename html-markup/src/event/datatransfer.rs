use crate::global::file::File;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataTransfer {
    pub drop_effect: DropEffect,
    pub effect_allowed: EffectAllowed,
    pub files: Vec<File>,
    pub items: Vec<DataTransferItem>,
    pub mime_types: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataTransferItem {
    pub content: DataTransferItemContent,
    pub mime_type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DataTransferItemContent {
    File(File),
    String(String),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DropEffect {
    None,
    Copy,
    Link,
    Move,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EffectAllowed {
    None,
    Copy,
    CopyLink,
    CopyMove,
    Link,
    LinkMove,
    Move,
    All,
    Uninitialized,
}