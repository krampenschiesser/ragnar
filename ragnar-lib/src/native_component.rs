use std::borrow::Cow;
use downcast_rs::{Downcast, impl_downcast};

use crate::callback::{NativeCallback, NativeCallbackWrapper, TypedCallbackRef};
use crate::TypedInputCallbackRef;
use crate::node::native_node::NativeNode;

pub trait NativeEvent: Downcast {
    fn get_type() -> &'static str where Self: Sized;
}
impl_downcast!(NativeEvent);

pub trait NativeComponent: NativeComponentWrapper {
    fn render(self, ctx: NativeContext) -> NativeNode;
}

pub trait NativeComponentWrapper {
    fn render(self) -> NativeNode;
}

impl<T: NativeComponent> NativeComponentWrapper for T {
    fn render(self) -> NativeNode {
        NativeComponent::render(self, NativeContext::new())
    }
}

pub struct NativeContext {
    callbacks: Vec<NativeCallbackWrapper>,
}

impl Default for NativeContext {
    fn default() -> Self {
        Self {
            callbacks: Vec::with_capacity(0),
        }
    }
}

impl NativeContext {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn forward(&self) -> Self {
        Self::default()
    }
    pub fn create_callback<T, In, Out: 'static>(&mut self, name: T, callback: impl Fn(In) -> Out + 'static) -> TypedCallbackRef<In, Out>
        where T: Into<Cow<'static, str>>, In: NativeEvent + 'static {
        let callback = NativeCallback::new(name, Box::new(callback));
        let callback_ref = callback.get_ref();
        self.callbacks.push(callback.into());
        callback_ref
    }
    pub fn create_chain<T, In>(&mut self, name: T, chained: TypedInputCallbackRef<In>)
        where T: Into<Cow<'static, str>>, In: NativeEvent {
        let callback = self.create_callback(name, |e| e);
        self.chain(callback, chained);
    }

    pub fn chain<In: 'static, T>(&mut self, callback: TypedCallbackRef<In, T>, chained: TypedInputCallbackRef<T>) {
        if let Some(callback) = self.callbacks.iter_mut().find(|c| c.id == callback.id) {
            callback.chained.push(chained.id);
        }
    }
    pub fn into_callbacks(self) -> Vec<NativeCallbackWrapper> {
        self.callbacks
    }
}