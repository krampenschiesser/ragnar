use std::borrow::Cow;

use crate::playground1::local_component::{LocalComponentWrapper};
use crate::playground1::native_component::NativeComponent;
use crate::playground1::callback::{CallbackWrapper};

pub struct Node {
    pub native_name: Option<Cow<'static, str>>,
    pub component: NodeComponentWrapper,
    pub children: NodeChildren,
    pub callbacks: Vec<CallbackWrapper>,
}

pub enum NodeComponentWrapper {
    Local(Box<dyn LocalComponentWrapper>),
    Native(Box<dyn NativeComponent>),
    None,
}

pub enum NodeChildren {
    Empty,
    Text(Cow<'static, str>),
    Nodes(Vec<Node>),
}