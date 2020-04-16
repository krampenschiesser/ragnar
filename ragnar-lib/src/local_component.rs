use std::any::Any;

use downcast_rs::{Downcast, impl_downcast};

use crate::callback::{LocalCallback, LocalCallbackWrapper, TypedCallbackRef};


use crate::node::local_node::LocalNode;
use std::marker::PhantomData;
use crate::TypedInputCallbackRef;

pub trait LocalEvent: Downcast {}
impl_downcast!(LocalEvent);

impl LocalEvent for () {}

pub trait LocalComponent: LocalComponentWrapper {
    type Msg: LocalEvent;

    fn render(self,  ctx: LocalContext<Self::Msg>) -> LocalNode;

    fn update(&self, msg: &Self::Msg, ctx: LocalContext<Self::Msg>) -> UpdateResult<Self>;
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
            self.update(event, LocalContext::new()).into()
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

pub struct LocalContext<T: LocalEvent> {
    callbacks: Vec<LocalCallbackWrapper>,
    _phantom: PhantomData<T>,
}

impl<T: LocalEvent> Default for LocalContext<T> {
    fn default() -> Self {
        Self {
            callbacks: Vec::with_capacity(0),
            _phantom: PhantomData,
        }
    }
}

impl<T: LocalEvent> LocalContext<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn forward(&self) -> Self {
        Self::default()
    }
    pub fn create_callback<In: 'static>(&mut self, callback: impl Fn(&In) -> T + 'static) -> TypedCallbackRef<In, T> {
        let callback = LocalCallback::new(Box::new(callback));
        let callback_ref = callback.get_ref();
        self.callbacks.push(callback.into());
        callback_ref
    }
    pub fn chain<In: 'static>(&mut self, callback: TypedCallbackRef<In, T>, chained: TypedInputCallbackRef<T>) {
        if let Some(callback) = self.callbacks.iter_mut().find(|c| c.id == callback.id) {
            callback.chained.push(chained.id);
        }
    }
    pub fn into_callbacks(self) -> Vec<LocalCallbackWrapper> {
        self.callbacks
    }
}