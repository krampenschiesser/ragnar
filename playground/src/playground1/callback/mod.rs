use std::any::Any;
use std::borrow::Cow;
use std::marker::PhantomData;

use crate::playground1::app_component::AppEvent;
use crate::playground1::INCREMENTER;
use crate::playground1::local_component::LocalEvent;
use crate::playground1::native_component::NativeEvent;
use crate::playground1::node::NodeId;

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

