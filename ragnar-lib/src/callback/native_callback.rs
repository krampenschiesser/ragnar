use crate::native_component::NativeEvent;
use crate::callback::{CallbackId, TypedInputCallbackRef};
use std::borrow::Cow;
use crate::INCREMENTER;
use std::marker::PhantomData;
use crate::node::NodeId;
use std::any::Any;

pub struct NativeCallback<In: NativeEvent + 'static, Out: 'static> {
    pub id: CallbackId,
    pub native_name: Cow<'static, str>,
    pub callback: Box<dyn Fn(In) -> Out>,
    pub chained: Vec<CallbackId>,
}

pub struct NativeCallbackWrapper {
    pub id: CallbackId,
    pub native_name: Cow<'static, str>,
    pub node_id: NodeId,
    pub callback: Box<dyn Fn(Box<dyn NativeEvent>) -> Option<Box<dyn Any>>>,
    pub chained: Vec<CallbackId>,
}

impl<In: NativeEvent, Out> NativeCallback<In, Out> {
    pub fn new<T: Into<Cow<'static, str>>>(name: T, callback: Box<dyn Fn(In) -> Out>) -> Self {
        Self {
            id: CallbackId(INCREMENTER.get_next()),
            native_name: name.into(),
            callback,
            chained: Vec::with_capacity(0),
        }
    }
    pub fn get_input_ref(&self) -> TypedInputCallbackRef<In> {
        TypedInputCallbackRef {
            id: self.id,
            _phantom: PhantomData,
        }
    }
    pub fn chain(&mut self, other: TypedInputCallbackRef<Out>) {
        self.chained.push(other.id);
    }
}

impl<In: NativeEvent + 'static, Out: 'static> Into<NativeCallbackWrapper> for NativeCallback<In, Out> {
    fn into(self) -> NativeCallbackWrapper {
        let id = self.id;
        let chained = self.chained;
        let callback = self.callback;
        let x = move |any: Box<dyn NativeEvent>| {
            if let Some(event) = any.downcast::<In>().ok() {
                let res: Out = (callback)(*event);
                let bo = Box::new(res) as Box<dyn Any>;
                Some(bo)
            } else {
                None
            }
        };
        NativeCallbackWrapper {
            id,
            native_name: self.native_name,
            node_id: NodeId(0),
            // callback_type,
            callback: Box::new(x),
            chained,
        }
    }
}