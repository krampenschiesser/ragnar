use actix::Message;

pub(crate) mod statelauncher;
pub(crate) mod state;
pub(crate) mod websocket;

#[derive(Debug,Clone,Copy)]
pub struct ShutDown;
impl Message for ShutDown {
    type Result = anyhow::Result<()>;
}
