use actix_web::{HttpServer, App, web};
use ragnar_lib::AppState;

use crate::error::Error;

mod command_line;
mod error;
mod app;

use crate::command_line::OperatingSystem;

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

pub async fn start<State: AppState>(initial_state: State) -> Result<(), Error> {
    println!("{:?}", OPERATING_SYSTEM);
    // HttpServer::new(|| {
    //     App::new()
    //         .route("/", web::get().to(index))
    //         .route("/again", web::get().to(index2))
    // })
    //     .bind("127.0.0.1:8088")?
    //     .run()
    //     .await?;
    Ok(())
}
