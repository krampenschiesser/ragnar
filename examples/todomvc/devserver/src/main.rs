use ragnar_dev_server_lib::start;
use ragnar_lib::NativeEvent;

#[tokio::main]
async fn main() {
    let initial_state = app::state::State::new();
    let component = app::component::app::App {};
    let x: Box<
        dyn Fn(&str, &str) -> Result<Option<Box<dyn NativeEvent>>, String> + Send + Sync + 'static,
    > = Box::new(ragnar_html_markup::resolve_native_event);
    let resolver = vec![x];
    let app = ragnar_lib::App::new(
        initial_state,
        component,
        Box::new(move |state, msg| {}),
        resolver,
    );
    start(app).await.unwrap()
}
