#[macro_use]
extern crate log;

use actix_web::{HttpServer, App, web, HttpRequest, HttpResponse};
use ragnar_lib::{AppState, AppComponent, AppEvent};
use anyhow::Result;
use std::sync::{Mutex, Arc, RwLock};

use crate::command_line::OperatingSystem;
use crate::actors::statelauncher::StateLauncher;
use actix::{Actor, Addr};
use crate::actors::websocket::WebsocketActor;

mod command_line;
mod error;
mod state_persistence;
pub(crate) mod actors;

pub(crate) trait StateExt: AppState + Clone + serde::Serialize + serde::de::DeserializeOwned + std::marker::Unpin + 'static {}

pub(crate) trait EventExt: AppEvent + Clone + serde::Serialize + serde::de::DeserializeOwned + std::marker::Unpin + 'static {}

impl<T> StateExt for T where T: AppState + Clone + serde::Serialize + serde::de::DeserializeOwned + std::marker::Unpin + 'static {}

impl<T> EventExt for T where T: AppEvent + Clone + serde::Serialize + serde::de::DeserializeOwned + std::marker::Unpin + 'static {}

#[cfg(feature = "web")]
pub static OPERATING_SYSTEM: OperatingSystem = OperatingSystem::Web;
#[cfg(feature = "ios")]
pub static OPERATING_SYSTEM: OperatingSystem = OperatingSystem::Ios;
#[cfg(feature = "android")]
pub static OPERATING_SYSTEM: OperatingSystem = OperatingSystem::Android;
#[cfg(feature = "windows")]
pub static OPERATING_SYSTEM: OperatingSystem = OperatingSystem::Windows;
#[cfg(feature = "linux")]
pub static OPERATING_SYSTEM: OperatingSystem = OperatingSystem::Linux;
#[cfg(feature = "mac")]
pub static OPERATING_SYSTEM: OperatingSystem = OperatingSystem::Mac;


pub async fn start<C, State, Msg>(app: ragnar_lib::App<C, State, Msg>) -> Result<()>
    where State: StateExt,
          Msg: EventExt,
          C: AppComponent<State=State, Msg=Msg> {
    let launcher: Addr<StateLauncher<State, Msg>> = StateLauncher::new(app.initial_state).start();


    println!("{:?}", OPERATING_SYSTEM);

    let data = actix_web::web::Data::new(launcher);
    HttpServer::new(move || {
        App::new().data(data.clone())
            .route("/ws", web::get().to(wsstart::<State, Msg>))
        // .route("/again", web::get().to(index2))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
        .map_err(|e| e.into())
}


async fn wsstart<State, Msg>(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::error::Error>
    where State: StateExt,
          Msg: EventExt {
    let data: &Addr<actors::statelauncher::StateLauncher<State, Msg>> = req.app_data().unwrap();
    let resp = actix_web_actors::ws::start(WebsocketActor { addr: data.clone(), state: None }, &req, stream);
    println!("{:?}", resp);
    resp
}