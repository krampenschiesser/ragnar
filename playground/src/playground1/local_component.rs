use downcast_rs::{Downcast, impl_downcast};

use crate::playground1::callback::{Callback, TypedInputCallbackRef};
use crate::playground1::INCREMENTER;
use crate::playground1::node::Node;

pub trait LocalEvent: Downcast {}
impl_downcast!(LocalEvent);

impl LocalEvent for () {}

pub trait LocalComponent: LocalComponentWrapper {
    type Msg: LocalEvent;

    fn render(self) -> Node;

    fn update(&mut self, msg: &Self::Msg) -> bool;

    fn create_local_callback<In>(callback: Box<dyn Fn(&In) -> Self::Msg>) -> Callback<In, Self::Msg> {
        Callback {
            id: INCREMENTER.get_next(),
            callback,
        }
    }
}

pub struct UpdateResult<T: LocalComponent> {
    should_render: bool,
    callbacks: Vec<TypedInputCallbackRef<T::Msg>>,
}

pub trait LocalComponentWrapper {
    fn handle(self, event: &dyn LocalEvent) -> LocalHandleResult;
}

impl<T: LocalComponent + 'static> LocalComponentWrapper for T {
    fn handle(mut self, event: &dyn LocalEvent) -> LocalHandleResult {
        if let Some(event) = event.downcast_ref::<T::Msg>() {
            let should_render = self.update(event);
            if should_render {
                LocalHandleResult::NewRender(self.render())
            } else {
                LocalHandleResult::NewState(Box::new(self))
            }
        } else {
            LocalHandleResult::NewState(Box::new(self))
        }
    }
}

pub enum LocalHandleResult {
    NewState(Box<dyn LocalComponentWrapper>),
    NewRender(Node),
}

