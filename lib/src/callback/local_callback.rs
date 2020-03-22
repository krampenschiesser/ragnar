use crate::playground1::callback::{CallbackId, TypedInputCallbackRef};
use crate::playground1::local_component::LocalEvent;
use crate::playground1::INCREMENTER;
use std::marker::PhantomData;
use crate::playground1::node::NodeId;
use std::any::Any;

pub struct LocalCallback<In: 'static, Out: LocalEvent + 'static> {
    pub id: CallbackId,
    pub callback: Box<dyn Fn(&In) -> Out>,
    pub chained: Vec<CallbackId>,
}

pub struct LocalCallbackWrapper {
    pub id: CallbackId,
    pub node_id: NodeId,
    pub callback: Box<dyn Fn(&Box<dyn Any>) -> Option<Box<dyn Any>>>,
    pub chained: Vec<CallbackId>,
}


impl<In, Out: LocalEvent> LocalCallback<In, Out> {
    pub fn new(callback: Box<dyn Fn(&In) -> Out>) -> Self {
        Self {
            id: CallbackId(INCREMENTER.get_next()),
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

impl<In: 'static, Out: LocalEvent + 'static> Into<LocalCallbackWrapper> for LocalCallback<In, Out> {
    fn into(self) -> LocalCallbackWrapper {
        let id = self.id;
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
        LocalCallbackWrapper {
            id,
            node_id: NodeId(0),
            // callback_type,
            callback: Box::new(x),
            chained,
        }
    }
}
