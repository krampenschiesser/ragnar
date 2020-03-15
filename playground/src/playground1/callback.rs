use std::any::Any;
use std::marker::PhantomData;

use crate::playground1::app_component::AppEvent;
use crate::playground1::INCREMENTER;
use crate::playground1::local_component::LocalEvent;
use crate::playground1::native_component::NativeEvent;
use crate::playground1::node::NodeId;
use std::borrow::Cow;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd,Hash)]
pub struct CallbackId(u64);

#[derive(Clone, Copy)]
pub struct TypedInputCallbackRef<In> {
    pub id: CallbackId,
    pub _phantom: std::marker::PhantomData<In>,
}

pub struct Callback<In: 'static, Out: 'static> {
    pub id: CallbackId,
    pub callback_type: CallbackType,
    pub callback: Box<dyn Fn(&In) -> Out>,
    pub chained: Vec<CallbackId>,
}

#[derive(Debug, Clone)]
pub enum CallbackType {
    Local,
    Native(Cow<'static, str>),
    App,
}

impl<In, Out: LocalEvent> Callback<In, Out> {
    pub fn new_local(callback: Box<dyn Fn(&In) -> Out>) -> Self {
        Self {
            id: CallbackId(INCREMENTER.get_next()),
            callback_type: CallbackType::Local,
            callback,
            chained: Vec::with_capacity(0),
        }
    }
}

impl<In: NativeEvent, Out> Callback<In, Out> {
    pub fn new_native<T: Into<Cow<'static, str>>>(name: T, callback: Box<dyn Fn(&In) -> Out>) -> Self {
        Self {
            id: CallbackId(INCREMENTER.get_next()),
            callback_type: CallbackType::Native(name.into()),
            callback,
            chained: Vec::with_capacity(0),
        }
    }
}

impl<In, Out: AppEvent> Callback<In, Out> {
    pub fn new_app(callback: Box<dyn Fn(&In) -> Out>) -> Self {
        Self {
            id: CallbackId(INCREMENTER.get_next()),
            callback_type: CallbackType::App,
            callback,
            chained: Vec::with_capacity(0),
        }
    }
}

impl<In, Out> Callback<In, Out> {
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

pub struct CallbackWrapper {
    pub callback_type: CallbackType,
    pub id: CallbackId,
    pub node_id: NodeId,
    pub callback: Box<dyn Fn(&Box<dyn Any>) -> Option<Box<dyn Any>>>,
    pub chained: Vec<CallbackId>,
}

impl<In: 'static, Out: 'static> Into<CallbackWrapper> for Callback<In, Out> {
    fn into(self) -> CallbackWrapper {
        let id = self.id;
        let callback_type = self.callback_type;
        let chained = self.chained;
        let callback = self.callback;
        let x = move |any: &Box<dyn Any>| {
            if let Some(event) = any.downcast_ref::<In>() {
                let res: Out = (callback)(event);
                let bo = Box::new(res) as Box<dyn Any>;
                Some(bo)
            } else {
                None
            }
        };
        CallbackWrapper {
            id,
            node_id: NodeId(0),
            callback_type,
            callback: Box::new(x),
            chained,
        }
    }
}
