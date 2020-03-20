use crate::playground1::callback::Callback;

use crate::playground1::node::{Node, TypedNode};
use downcast_rs::{impl_downcast,Downcast};

pub trait AppState {}

pub trait AppEvent: Downcast + 'static {}
impl_downcast!(AppEvent);

pub trait AppComponent {
    type Msg: AppEvent;
    type State: AppState;

    fn render(&self, state: &Self::State) -> TypedNode<Self::Msg>;

    fn create_app_callback<In: 'static>(callback: Box<dyn Fn(&In) -> Self::Msg>) -> Callback<In, Self::Msg> {
        Callback::new_app(callback)
    }
}
