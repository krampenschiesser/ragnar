use std::any::Any;

use downcast_rs::{Downcast, impl_downcast};

use crate::playground1::callback::{LocalCallback};

use crate::playground1::node::Node;
use crate::playground1::node::local_node::LocalNode;

pub trait LocalEvent: Downcast {}
impl_downcast!(LocalEvent);

impl LocalEvent for () {}

pub trait LocalComponent: LocalComponentWrapper {
    type Msg: LocalEvent;

    fn render(self) -> LocalNode;

    fn update(&self, msg: &Self::Msg) -> UpdateResult<Self>;

    fn create_local_callback<In>(callback: Box<dyn Fn(&In) -> Self::Msg>) -> LocalCallback<In, Self::Msg> {
        LocalCallback::new(callback)
    }
}

pub enum UpdateResult<T: LocalComponent + ?Sized> {
    NewRender(LocalNode),
    NewState(Box<T>),
    Keep,
}

impl<T: LocalComponent> From<LocalNode> for UpdateResult<T> {
    fn from(n: LocalNode) -> Self {
        UpdateResult::NewRender(n)
    }
}

impl<T: LocalComponent> From<T> for UpdateResult<T> {
    fn from(n: T) -> Self {
        UpdateResult::NewState(Box::new(n))
    }
}
impl<T: LocalComponent> From<Option<T>> for UpdateResult<T> {
    fn from(n: Option<T>) -> Self {
        match n {
            Some(t) =>UpdateResult::NewState(Box::new(t)),
            None => UpdateResult::Keep,
        }
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
    NewRender(LocalNode),
    Keep,
}

impl<T: LocalComponent + 'static> From<UpdateResult<T>> for LocalHandleResult {
    fn from(t: UpdateResult<T>) -> Self {
        match t {
            UpdateResult::NewState(t) => LocalHandleResult::NewState(t as Box<dyn LocalComponentWrapper>),
            UpdateResult::NewRender(node) => LocalHandleResult::NewRender(node),
            UpdateResult::Keep => LocalHandleResult::Keep,
        }
    }
}
