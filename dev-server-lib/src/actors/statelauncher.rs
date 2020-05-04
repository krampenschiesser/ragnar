use actix::prelude::*;
use std::collections::HashMap;
use crate::actors::state::StateActor;
use ragnar_lib::{AppState, AppEvent};
use crate::state_persistence::SingleState;
use crate::actors::ShutDown;
use crate::actors::websocket::{IncomingClientMessageWithAddress, SessionId};
use crate::{StateExt, EventExt};
use actors_rs::Sender;

pub struct StateLauncher<State: StateExt, Msg: EventExt> {
    states: HashMap<SessionId, Addr<StateActor<State, Msg>>>,
    initial: State,
}

impl<State: StateExt, Msg: EventExt> StateLauncher<State, Msg> {
    pub fn new(initial: State) -> Self {
        Self {
            states: HashMap::new(),
            initial,
        }
    }
}


impl<State: StateExt, Msg: EventExt> actors_rs::Actor for StateLauncher<State, Msg> {
    type Msg = ();

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
        unimplemented!()
    }
}
impl<State: StateExt, Msg: EventExt> Actor for StateLauncher<State, Msg> {
    type Context = Context<Self>;
}

pub struct StartClient(pub SessionId);


impl Message for StartClient {
    type Result = anyhow::Result<u8>;
}

impl<State: StateExt, Msg: EventExt> Handler<StartClient> for StateLauncher<State, Msg> {
    type Result = anyhow::Result<u8>;

    fn handle(&mut self, msg: StartClient, ctx: &mut Context<Self>) -> Self::Result {
        if let Some(addr) = self.states.get(&msg.0) {
            // addr.
            Ok(0)
        }else {
            let addr = StateActor::load(msg.0.clone(), self.initial.clone())?.start();
            self.states.insert(msg.0, addr);
            Ok(0)
        }
    }
}

impl<State: StateExt, Msg: EventExt> Handler<ShutDown> for StateLauncher<State, Msg> {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: ShutDown, ctx: &mut Context<Self>) -> Self::Result {
        for (name, addr) in self.states.iter() {
            addr.send(msg);
        }
        Ok(())
    }
}

impl<State: StateExt, Msg: EventExt> Handler<IncomingClientMessageWithAddress<State,Msg>> for StateLauncher<State, Msg> {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: IncomingClientMessageWithAddress<State,Msg>, ctx: &mut Context<Self>) -> Self::Result {

        for (name, addr) in self.states.iter() {
            addr.send(msg);
        }
        Ok(())
    }
}