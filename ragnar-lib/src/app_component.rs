use crate::callback::{AppCallback, AppCallbackWrapper, TypedCallbackRef};

use crate::node::app_node::AppNode;
use crate::TypedInputCallbackRef;
use downcast_rs::{impl_downcast, Downcast};
use std::marker::PhantomData;

pub trait AppState {}

pub trait AppEvent: Downcast + 'static + Unpin {}
impl_downcast!(AppEvent);

impl AppState for () {}

pub trait AppComponent {
    type Msg: AppEvent;
    type State: AppState;

    fn render(&self, state: &Self::State, ctx: AppContext<Self::Msg>) -> AppNode<Self::Msg>;
}

pub struct AppContext<T: AppEvent> {
    callbacks: Vec<AppCallbackWrapper>,
    _phantom: PhantomData<T>,
}

impl<T: AppEvent> Default for AppContext<T> {
    fn default() -> Self {
        Self {
            callbacks: Vec::with_capacity(0),
            _phantom: PhantomData,
        }
    }
}

impl<T: AppEvent> AppContext<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn forward(&self) -> Self {
        Self::default()
    }
    pub fn create_callback<In: 'static>(
        &mut self,
        callback: impl FnOnce(&In) -> T + 'static,
    ) -> TypedCallbackRef<In, T> {
        let callback = AppCallback::new(Box::new(callback));
        let callback_ref = callback.get_ref();
        self.callbacks.push(callback.into());
        callback_ref
    }
    pub fn chain<In: 'static>(
        &mut self,
        callback: TypedCallbackRef<In, T>,
        chained: TypedInputCallbackRef<T>,
    ) {
        if let Some(callback) = self.callbacks.iter_mut().find(|c| c.id == callback.id) {
            callback.chained.push(chained.id);
        }
    }
    pub fn into_callbacks(self) -> Vec<AppCallbackWrapper> {
        self.callbacks
    }
}
