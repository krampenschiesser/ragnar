use std::borrow::Cow;

use crate::playground1::callback::{NativeCallback};


use crate::playground1::node::native_node::NativeNode;
use downcast_rs::{Downcast,impl_downcast};

pub trait NativeEvent: Downcast {}
impl_downcast!(NativeEvent);

pub trait NativeComponent: NativeComponentWrapper {
    fn render(self) -> NativeNode;
    fn create_native_callback<T, In, Out>(name: T, callback: Box<dyn Fn(In) -> Out>) -> NativeCallback<In, Out>
        where T: Into<Cow<'static, str>>, In: NativeEvent {
        NativeCallback::new(name, callback)
    }
}

pub trait NativeComponentWrapper {
    fn render(self) -> NativeNode;
}

impl<T: NativeComponent> NativeComponentWrapper for T {
    fn render(self) -> NativeNode {
        NativeComponent::render(self)
    }
}