#[macro_use]
extern crate log;

use anyhow::Result;
use ragnar_lib::{App, AppComponent, AppEvent, AppState};

use crate::messages::{WebsocketRequest, WebsocketResponse};
use crate::state::state_container::StateContainer;
use futures::lock::Mutex;
use std::sync::Arc;
use warp::filters::ws::Message;
use warp::ws::WebSocket;

mod actor;
mod command_line;
mod error;
mod messages;
mod os;
mod state;
mod state_persistence;

pub trait StateExt:
    AppState + Clone + serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static
{
}

pub trait EventExt:
    AppEvent + Clone + serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static
{
}

impl<T> StateExt for T where
    T: AppState + Clone + serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static
{
}

impl<T> EventExt for T where
    T: AppEvent + Clone + serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static
{
}

pub async fn start<C, State, Msg>(app: App<C, State, Msg>) -> Result<()>
where
    State: StateExt,
    Msg: EventExt,
    C: AppComponent<State = State, Msg = Msg> + Clone + Send + Sync + 'static,
{
    use warp::Filter;
    let state = Arc::new(Mutex::new(StateContainer::new(app.clone())));

    let routes = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let state = state.clone();
            ws.on_upgrade(move |websocket| handle_new_websocket(websocket, state))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

async fn handle_new_websocket<C, S, M>(
    websocket: WebSocket,
    state: Arc<Mutex<StateContainer<C, S, M>>>,
) where
    S: StateExt,
    M: EventExt,
    C: AppComponent<State = S, Msg = M> + Send + Sync + Clone,
{
    match handle_new_websocket_internal(websocket, state).await {
        Err(e) => error!("Error handling websocket {}", e),
        Ok(_) => (),
    }
}

async fn handle_new_websocket_internal<C, S, M>(
    websocket: WebSocket,
    state: Arc<Mutex<StateContainer<C, S, M>>>,
) -> anyhow::Result<()>
where
    S: StateExt,
    M: EventExt,
    C: AppComponent<State = S, Msg = M> + Send + Sync + Clone,
{
    use futures::{FutureExt, SinkExt, StreamExt};

    let (mut tx, mut rx) = websocket.split();

    let client_id = {
        let state = state.lock().await;

        state.next_client_id()
    };

    while let Some(msg) = rx.next().await {
        let msg = msg?;
        if msg.is_text() {
            let req: WebsocketRequest = serde_json::from_str(msg.to_str().unwrap())?;
            match req {
                WebsocketRequest::JoinSession(session) => {
                    let guard = state.lock().await;
                    let res = guard.join_session(session, client_id).await?;

                    let mut items = vec![
                        Message::text(serde_json::to_string(&WebsocketResponse::Registered(
                            client_id,
                        ))?),
                        Message::text(serde_json::to_string(&WebsocketResponse::Clear)?),
                        Message::text(serde_json::to_string(&WebsocketResponse::Diff(res))?),
                    ];
                    let mut stream = futures::stream::iter(items.into_iter()).map(Ok);
                    tx.send_all(&mut stream).await?;
                }
                WebsocketRequest::NativeEvent {
                    session,
                    callback_id,
                    event_type,
                    payload,
                } => {
                    let guard = state.lock().await;
                    let res = guard
                        .handle_native_event(session, client_id, callback_id, event_type, payload)
                        .await?;
                    tx.send(Message::text(serde_json::to_string(
                        &WebsocketResponse::Diff(res),
                    )?))
                    .await?
                }
            }
        }
    }
    Ok(())
}
