use actix::{Actor, StreamHandler, Addr, Message, AsyncContext};
use actix_web_actors::ws;
use ragnar_lib::{AppState, AppEvent, DiffOperation};
use crate::actors::statelauncher::{StateLauncher, StartClient};
use crate::{StateExt, EventExt};

pub struct WebsocketActor<State: StateExt, Msg: EventExt> {
    pub addr: Addr<StateLauncher<State, Msg>>,
    pub state: Option<InternalState>,
}

struct InternalState {
    client_id: u8,
    session: SessionId,
}

impl<State: StateExt, Msg: EventExt> Actor for WebsocketActor<State, Msg> {
    type Context = ws::WebsocketContext<Self>;
}

impl<State: StateExt, Msg: EventExt> StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketActor<State, Msg> {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let res = self.handle_msg(text.as_str(), ctx.address());
                match res {
                    Ok(response) => (),
                    Err(e) => error!("Could not handle {}: {}", text, e),
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl<State: StateExt, Msg: EventExt> WebsocketActor<State, Msg> {
    async fn handle_msg(&mut self, text: &str, addr: Addr<WebsocketActor<State, Msg>>) -> anyhow::Result<()> {
        let msg: IncomingClientMessage = serde_json::from_str(text)?;
        if self.state.is_none() {
            let client_id = self.addr.send(StartClient(msg.get_session().clone())).await??;
            self.state = Some(InternalState {
                session: msg.get_session().clone(),
                client_id,
            });
        }
        if let Some(state) = &self.state {
            let msg = IncomingClientMessageWithAddress {
                addr,
                client_id: state.client_id,
                msg,
            };
            self.addr.send(msg).await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Hash)]
pub struct ClientId(pub String);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Hash, Eq, PartialEq)]
pub struct SessionId(pub String);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum IncomingClientMessage {
    SwitchSession(SessionId),
    NativeEvent {
        session: SessionId,
        id: u64,
        event_type: String,
        payload: String,
    },
}

impl IncomingClientMessage {
    pub fn get_session(&self) -> &SessionId {
        match self {
            IncomingClientMessage::SwitchSession(s) => s,
            IncomingClientMessage::NativeEvent { session, id, event_type, payload } => session,
        }
    }
}


pub struct IncomingClientMessageWithAddress<State: StateExt, Msg: EventExt> {
    msg: IncomingClientMessage,
    addr: Addr<WebsocketActor<State, Msg>>,
    client_id: u8,
}

impl<State: StateExt, Msg: EventExt> Message for IncomingClientMessageWithAddress<State, Msg> {
    type Result = anyhow::Result<()>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum OutgoingClientMessage {
    Registered(u8),
    Clear,
    DiffOperations(Vec<DiffOperation>),
}