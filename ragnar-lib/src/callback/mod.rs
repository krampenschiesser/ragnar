use std::any::Any;







use crate::node::NodeId;

mod native_callback;
mod local_callback;
mod app_callback;

pub use native_callback::{NativeCallback,NativeCallbackWrapper};
pub use local_callback::{LocalCallback,LocalCallbackWrapper};
pub use app_callback::{AppCallback,AppCallbackWrapper};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CallbackId(u64);

#[derive(Clone, Copy)]
pub struct TypedInputCallbackRef<In> {
    pub id: CallbackId,
    pub _phantom: std::marker::PhantomData<In>,
}



pub struct CallbackWrapper {
    // pub callback_type: CallbackType,
    pub id: CallbackId,
    pub node_id: NodeId,
    pub callback: Box<dyn Fn(&Box<dyn Any>) -> Option<Box<dyn Any>>>,
    pub chained: Vec<CallbackId>,
}
