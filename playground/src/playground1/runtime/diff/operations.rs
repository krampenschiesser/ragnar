use std::borrow::Cow;
use std::collections::HashMap;

use crate::playground1::attribute::Attribute;
use crate::playground1::callback::{CallbackId, CallbackType, CallbackWrapper};
use crate::playground1::node::NodeId;
use crate::playground1::runtime::denormalized_node::{NodeContainer, StrippedNode};
use crate::playground1::runtime::diff::DiffError;

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
    pub fn may_from(callback: &CallbackWrapper) -> Option<Self> {
        match &callback.callback_type {
            CallbackType::Native(native_name) => {
                Some(
                    Self {
                        callback_name: native_name.clone(),
                        node_id: callback.node_id,
                        id: callback.id,
                    }
                )
            }
            _ => None
        }
    }
}

impl AddNode {
    pub fn new(node: &StrippedNode, container: &NodeContainer, parent: Option<ParentPosition>) -> Result<Self, DiffError> {
        let native_name = node.native_name.clone().ok_or(DiffError::NonNative(node.id))?;
        let callbacks: Vec<SetListener> = node.callbacks.iter()
            .filter_map(|cid| container.get_callback(cid))
            .filter_map(|c| SetListener::may_from(c))
            .collect();

        let attributes = node.attributes.clone();
        Ok(Self {
            parent,
            node_id: node.id,
            native_name,
            callbacks,
            attributes,
        })
    }
}

pub struct ParentPosition {
    pub parent: NodeId,
    pub index: u64,
}

impl SwapNode {
    pub fn new(original: NodeId, node: &StrippedNode, container: &NodeContainer, parent: Option<ParentPosition>) -> Result<Self, DiffError> {
        let new = AddNode::new(node, container, parent)?;
        Ok(Self {
            original,
            new,
        })
    }
}