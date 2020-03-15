use std::any::Any;

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

    fn update(&self, msg: &Self::Msg) -> UpdateResult<Self>;

    fn create_local_callback<In>(callback: Box<dyn Fn(&In) -> Self::Msg>) -> Callback<In, Self::Msg> {
        Callback::new_local(callback)
    }
}

pub enum UpdateResult<T: LocalComponent + ?Sized> {
    NewRender(Node),
    NewState(Box<T>),
    Nothing,
}

impl<T: LocalComponent> From<Node> for UpdateResult<T> {
    fn from(n: Node) -> Self {
        UpdateResult::NewRender(n)
    }
}

impl<T: LocalComponent> From<T> for UpdateResult<T> {
    fn from(t: T) -> Self {
        UpdateResult::NewState(Box::new(t))
    }
}

pub trait LocalComponentWrapper {
    fn handle(&self, event: &Box<dyn Any>) -> LocalHandleResult;
}

impl<T: LocalComponent + 'static> LocalComponentWrapper for T {
    fn handle(&self, event: &Box<dyn Any>) -> LocalHandleResult {
        if let Some(event) = event.downcast_ref::<T::Msg>() {
            self.update(event).into()
        } else {
            LocalHandleResult::Keep
        }
    }
}

pub enum LocalHandleResult {
    NewState(Box<dyn LocalComponentWrapper>),
    NewRender(Node),
    Keep,
}

impl<T: LocalComponent + 'static> From<UpdateResult<T>> for LocalHandleResult {
    fn from(t: UpdateResult<T>) -> Self {
        match t {
            UpdateResult::NewState(t) => LocalHandleResult::NewState(t as Box<dyn LocalComponentWrapper>),
            UpdateResult::NewRender(node) => LocalHandleResult::NewRender(node),
            UpdateResult::Nothing => LocalHandleResult::Keep,
        }
    }
}
