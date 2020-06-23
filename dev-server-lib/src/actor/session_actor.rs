use crate::messages::{SessionRequest, SessionResponse};
use crate::state::{ClientId, SessionId};
use crate::{EventExt, StateExt};
use ragnar_lib::{App, AppComponent, CallbackId, DiffOperation, Runtime};

pub struct SessionActor<
    C: AppComponent<State = State, Msg = Msg> + Clone + 'static,
    State: StateExt,
    Msg: EventExt,
> {
    runtime: Runtime<C, State, Msg>,
    session: SessionId,
    clients: Vec<ClientId>,
    receiver: std::sync::mpsc::Receiver<(
        SessionRequest,
        tokio::sync::oneshot::Sender<SessionResponse>,
    )>,
}

impl<
        C: AppComponent<State = State, Msg = Msg> + Clone + 'static,
        State: StateExt,
        Msg: EventExt,
    > SessionActor<C, State, Msg>
{
    pub fn new(
        app: &App<C, State, Msg>,
        session: SessionId,
    ) -> (
        Self,
        std::sync::mpsc::SyncSender<(
            SessionRequest,
            tokio::sync::oneshot::Sender<SessionResponse>,
        )>,
    ) {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1024);
        let runtime = Runtime::new(app);
        (
            Self {
                session,
                runtime,
                clients: Vec::new(),
                receiver,
            },
            sender,
        )
    }

    pub fn handle(mut self) -> anyhow::Result<()> {
        let error = "Could not send message, target not available";
        while let (cmd, response) = self.receiver.recv()? {
            match cmd {
                SessionRequest::AddClient(client_id) => {
                    self.clients.push(client_id);
                    response
                        .send(SessionResponse::Ok)
                        .map_err(|_| anyhow::Error::msg(error))?;
                }
                SessionRequest::InitialDiff(client_id) => {
                    let vec = self.runtime.initial_diff();
                    response
                        .send(SessionResponse::DiffResult(vec))
                        .map_err(|_| anyhow::Error::msg(error))?;
                }
                SessionRequest::NativeEvent {
                    session_id,
                    client_id,
                    callback_id,
                    event_type,
                    payload,
                } => {
                    let event = self
                        .runtime
                        .resolve_native_event(&event_type, &payload)
                        .map_err(|e| anyhow::Error::msg(e))?;
                    let diff = self.runtime.handle_event(callback_id, event)?;
                    response
                        .send(SessionResponse::DiffResult(diff))
                        .map_err(|_| anyhow::Error::msg(error))?;
                }
            }
        }
        Ok(())
    }
}
