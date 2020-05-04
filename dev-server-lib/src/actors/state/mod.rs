use actix::prelude::*;
use ragnar_lib::{AppState, AppEvent};
use crate::state_persistence::SingleState;
use crate::actors::ShutDown;
use crate::actors::websocket::{SessionId, IncomingClientMessageWithAddress};
use crate::{StateExt, EventExt};

pub struct StateActor<State: StateExt, Msg: EventExt> {
    state: SingleState<State, Msg>,
    session: SessionId,
}


impl<State: StateExt, Msg: EventExt> Actor for StateActor<State, Msg> {
    type Context = Context<Self>;
}

impl<State: StateExt, Msg: EventExt> StateActor<State, Msg> {
    pub fn load(session: SessionId, state: State) -> anyhow::Result<Self> {
        let state: SingleState<State, Msg> = SingleState::load(&session.0, state)?;
        Ok(Self { state, session })
    }
}

impl<State: StateExt, Msg: EventExt> Handler<ShutDown> for StateActor<State, Msg> {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: ShutDown, ctx: &mut Context<Self>) -> Self::Result {
        self.state.save(&self.session.0)?;
        Ok(())
    }
}

impl<State: StateExt, Msg: EventExt> Handler<IncomingClientMessageWithAddress<State,Msg>> for StateActor<State, Msg> {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: IncomingClientMessageWithAddress<State,Msg>, ctx: &mut Context<Self>) -> Self::Result {
        // self.state.save(&self.session.0)?;
        Ok(())
    }
}

