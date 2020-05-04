use ragnar_dev_server_lib::start;

#[actix_rt::main]
async fn main() -> Result<(),anyhow::Error> {
    let initial_state = app::state::State::new();
    let component = app::component::app::App{};
    let app = ragnar_lib::App {
        initial_state,
        root_component: component,
        update_function: Box::new(move |state,msg|{})
    };
    start(app).await
}