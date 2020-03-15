use crate::playground1::callback::Callback;
use crate::playground1::INCREMENTER;
use crate::playground1::node::Node;

pub trait AppState {}

pub trait AppEvent: 'static {}

pub trait AppComponent {
    type Msg: AppEvent;
    type State: AppState;

    fn render(&self, state: &Self::State) -> Node;

    fn create_app_callback<In: 'static>(callback: Box<dyn Fn(&In) -> Self::Msg>) -> Callback<In, Self::Msg> {
        Callback::new_app(callback)
    }
}