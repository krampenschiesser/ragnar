use super::ClientId;
use crate::actor::session_actor::SessionActor;
use crate::messages::{SessionRequest, SessionResponse};
use crate::state::SessionId;
use crate::{EventExt, StateExt};
use futures::lock::Mutex;
use ragnar_lib::{App, AppComponent, CallbackId, DiffOperation};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

pub struct StateContainer<
    C: AppComponent<State = State, Msg = Msg> + Clone + Send + Sync + 'static,
    State: StateExt,
    Msg: EventExt,
> {
    actors: Arc<
        Mutex<
            HashMap<
                SessionId,
                std::sync::mpsc::SyncSender<(
                    SessionRequest,
                    tokio::sync::oneshot::Sender<SessionResponse>,
                )>,
            >,
        >,
    >,
    app: App<C, State, Msg>,
    client_id_counter: AtomicU32,
}

impl<
        C: AppComponent<State = State, Msg = Msg> + Send + Sync + Clone,
        State: StateExt,
        Msg: EventExt,
    > StateContainer<C, State, Msg>
{
    pub fn new(app: App<C, State, Msg>) -> Self {
        Self {
            actors: Arc::new(Mutex::new(HashMap::new())),
            client_id_counter: AtomicU32::new(0),
            app,
        }
    }

    pub fn next_client_id(&self) -> ClientId {
        ClientId(self.client_id_counter.fetch_add(1, Ordering::SeqCst))
    }

    async fn create_actor_if_not_exists(&self, session_id: SessionId) -> anyhow::Result<()> {
        let mut map = self.actors.lock().await;
        let app = self.app.clone();
        if !map.contains_key(&session_id) {
            let (sender, receiver) = tokio::sync::oneshot::channel();
            std::thread::spawn(move || {
                let (actor, send) = SessionActor::new(&app, SessionId("bla".into()));
                sender.send(send);
                actor.handle();
            });
            let res = receiver.await?;
            map.insert(session_id.clone(), res);
        }
        Ok(())
    }

    pub async fn join_session(
        &self,
        session_id: SessionId,
        client_id: ClientId,
    ) -> anyhow::Result<Vec<DiffOperation>> {
        self.create_actor_if_not_exists(session_id.clone()).await?;

        let _r: SessionResponse = self
            .send(&session_id, SessionRequest::AddClient(client_id))
            .await?;
        let diff_result = self
            .send(&session_id, SessionRequest::InitialDiff(client_id))
            .await?;
        match diff_result {
            SessionResponse::DiffResult(vec) => Ok(vec),
            _ => Err(anyhow::Error::msg(format!(
                "Invalid response {:?}",
                diff_result
            ))),
        }
    }

    pub async fn handle_native_event(
        &self,
        session_id: SessionId,
        client_id: ClientId,
        callback_id: CallbackId,
        event_type: String,
        payload: String,
    ) -> anyhow::Result<Vec<DiffOperation>> {
        self.create_actor_if_not_exists(session_id.clone()).await?;

        let diff_result: SessionResponse = self
            .send(
                &session_id,
                SessionRequest::NativeEvent {
                    session_id: session_id.clone(),
                    client_id,
                    callback_id,
                    event_type,
                    payload,
                },
            )
            .await?;
        match diff_result {
            SessionResponse::DiffResult(vec) => Ok(vec),
            _ => Err(anyhow::Error::msg(format!(
                "Invalid response {:?}",
                diff_result
            ))),
        }
    }

    async fn send(
        &self,
        session: &SessionId,
        req: SessionRequest,
    ) -> anyhow::Result<SessionResponse> {
        let map = self.actors.lock().await;

        let error_creator =
            || anyhow::Error::msg(format!("Could not find actor for session {:?}", session));
        let sender = map.get(session).ok_or_else(error_creator)?;
        let (send, receiver) = tokio::sync::oneshot::channel();
        sender.send((req, send))?;
        let response = receiver.await?;
        Ok(response)
    }

    // pub async execute_native_event
}
