use std::ops::Deref;

use crate::playground1::node::app_node::UntypedAppNode;
use crate::playground1::node::local_node::LocalNode;
use crate::playground1::node::native_node::NativeNode;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct NodeId(pub u64);

impl Deref for NodeId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub mod app_node;
pub mod native_node;
pub mod local_node;

pub enum Node {
    Native(NativeNode),
    Local(LocalNode),
    App(UntypedAppNode),
}

impl Node {
    pub fn get_id(&self) -> NodeId {
        match self {
            Node::Native(n) => n.id,
            Node::Local(n) => n.id,
            Node::App(n) => n.id,
        }
    }
}