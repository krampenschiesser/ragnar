use std::ops::Deref;

pub mod state_container;

#[derive(Debug, Eq, Hash, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClientId(pub u32);

#[derive(Debug, Eq, Hash, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SessionId(pub String);

impl std::fmt::Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Deref for SessionId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
