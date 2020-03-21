use std::borrow::Cow;
use std::collections::HashMap;

use crate::playground1::attribute::Attribute;
use crate::playground1::callback::{CallbackId, NativeCallbackWrapper};
use crate::playground1::node::NodeId;
use crate::playground1::runtime::diff::DiffError;
use crate::playground1::runtime::node_container::{NativeView, NodeContainer};
use crate::playground1::runtime::node_container::stripped_node::StrippedNativeNode;

pub struct SetAttribute {
    pub node_id: NodeId,
    pub attribute_name: Cow<'static, str>,
    pub attribute_value: Attribute,
}

pub struct RemoveAttribute {
    pub node_id: NodeId,
    pub attribute_name: Cow<'static, str>,
}

pub struct RemoveListener {
    pub node_id: NodeId,
    pub callback_id: CallbackId,
}

pub struct SetListener {
    pub callback_name: Cow<'static, str>,
    pub id: CallbackId,
    pub node_id: NodeId,
}

pub struct RemoveNode {
    pub node_id: NodeId
}

pub struct AddNode {
    pub parent: Option<ParentPosition>,
    pub node_id: NodeId,
    pub native_name: Cow<'static, str>,
    pub callbacks: Vec<SetListener>,
    pub attributes: HashMap<Cow<'static, str>, Attribute>,
}

pub struct SwapNode {
    pub original: NodeId,
    pub new: AddNode,
}

pub struct SwapNodeId {
    pub original: NodeId,
    pub new: NodeId,
}

pub enum DiffOperation {
    SetAttribute(SetAttribute),
    RemoveAttribute(RemoveAttribute),

    SetListener(SetListener),
    RemoveListener(RemoveListener),

    RemoveNode(RemoveNode),
    AddNode(AddNode),
    SwapNode(SwapNode),
    SwapNodeId(SwapNodeId),
}

impl SetListener {
    pub fn from(callback: &NativeCallbackWrapper) -> Self {
        Self {
            callback_name: callback.native_name.clone(),
            node_id: callback.node_id,
            id: callback.id,
        }
    }
}

impl AddNode {
    pub fn new(node: &NativeView, parent: Option<ParentPosition>) -> Self {
        let callbacks: Vec<SetListener> = node.callbacks.iter()
            .map(|c| SetListener::from(c))
            .collect();

        let node = node.node;
        let attributes = node.attributes.clone();
        Self {
            parent,
            node_id: node.id,
            native_name: node.native_name.clone(),
            callbacks,
            attributes,
        }
    }
}
#[derive(Debug,Clone,Copy)]
pub struct ParentPosition {
    pub parent: NodeId,
    pub index: u64,
}

impl SwapNode {
    pub fn new(original: NodeId, node: &NativeView, parent: Option<ParentPosition>) -> Self {
        let new = AddNode::new(node, parent);
        Self {
            original,
            new,
        }
    }
}