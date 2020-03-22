use crate::playground1::callback::{AppCallback};


use downcast_rs::{impl_downcast,Downcast};
use crate::playground1::node::app_node::AppNode;

pub trait AppState {}

pub trait AppEvent: Downcast + 'static {}
impl_downcast!(AppEvent);

pub trait AppComponent {
    type Msg: AppEvent;
    type State: AppState;

    fn render(&self, state: &Self::State) -> AppNode<Self::Msg>;

    fn create_app_callback<In: 'static>(callback: Box<dyn Fn(&In) -> Self::Msg>) -> AppCallback<In, Self::Msg> {
        AppCallback::new(callback)
    }
}
