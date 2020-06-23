pub mod state_container;

#[derive(Debug, Eq, Hash, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClientId(pub u32);

#[derive(Debug, Eq, Hash, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SessionId(pub String);
