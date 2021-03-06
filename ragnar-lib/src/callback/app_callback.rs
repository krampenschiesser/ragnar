use crate::callback::{CallbackId, TypedInputCallbackRef, TypedCallbackRef};
use crate::app_component::AppEvent;
use crate::INCREMENTER;
use std::marker::PhantomData;
use std::any::Any;
use crate::node::NodeId;
use std::cell::Cell;

pub struct AppCallback<In: 'static, Out: AppEvent + 'static> {
    pub id: CallbackId,
    pub callback: Box<dyn FnOnce(&In) -> Out>,
    pub chained: Vec<CallbackId>,
}

pub struct AppCallbackWrapper {
    pub id: CallbackId,
    pub node_id: NodeId,
    pub callback: Cell<Option<Box<dyn FnOnce(&Box<dyn Any>) -> Option<Box<dyn Any>>>>>,
    pub chained: Vec<CallbackId>,
}

impl<In, Out: AppEvent> AppCallback<In, Out> {
    pub fn new(callback: Box<dyn FnOnce(&In) -> Out>) -> Self {
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
    pub fn get_ref(&self) -> TypedCallbackRef<In,Out> {
        TypedCallbackRef {
            id: self.id,
            _out: PhantomData,
            _in: PhantomData,
        }
    }
    pub fn chain(&mut self, other: TypedInputCallbackRef<Out>) {
        self.chained.push(other.id);
    }
}

impl<In: 'static, Out: AppEvent + 'static> Into<AppCallbackWrapper> for AppCallback<In, Out> {
    fn into(self) -> AppCallbackWrapper {
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
        AppCallbackWrapper {
            id,
            node_id: NodeId(0),
            // callback_type,
            callback: Cell::new(Some(Box::new(x))),
            chained,
        }
    }
}
