use crate::state::{ClientId, SessionId};
use ragnar_lib::{CallbackId, DiffOperation};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum WebsocketRequest {
    JoinSession(SessionId),
    NativeEvent {
        session: SessionId,
        callback_id: CallbackId,
        event_type: String,
        payload: String,
    },
    ListSessions,
    GetSingleState(SessionId),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum WebsocketResponse {
    Registered(ClientId),
    Clear,
    Diff(Vec<DiffOperation>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SessionRequest {
    AddClient(ClientId),
    InitialDiff(ClientId),
    NativeEvent {
        session_id: SessionId,
        client_id: ClientId,
        callback_id: CallbackId,
        event_type: String,
        payload: String,
    },
    Quit,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SessionResponse {
    Ok,
    DiffResult(Vec<DiffOperation>),
}
