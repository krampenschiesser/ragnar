use std::borrow::Cow;
use std::collections::HashMap;

use crate::attribute::Attribute;
use crate::callback::{CallbackId, NativeCallbackWrapper};
use crate::node::NodeId;

use crate::runtime::node_container::NativeView;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetAttribute {
    pub node_id: NodeId,
    pub attribute_name: Cow<'static, str>,
    pub attribute_value: Attribute,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoveAttribute {
    pub node_id: NodeId,
    pub attribute_name: Cow<'static, str>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoveListener {
    pub node_id: NodeId,
    pub callback_id: CallbackId,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetListener {
    pub callback_name: Cow<'static, str>,
    pub id: CallbackId,
    pub node_id: NodeId,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoveNode {
    pub node_id: NodeId
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddNode {
    pub parent: Option<ParentPosition>,
    pub node_id: NodeId,
    pub native_name: Cow<'static, str>,
    pub text: Option<Cow<'static, str>>,
    pub callbacks: Vec<SetListener>,
    pub attributes: HashMap<Cow<'static, str>, Attribute>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SwapNode {
    pub original: NodeId,
    pub new: AddNode,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SwapNodeId {
    pub original: NodeId,
    pub new: NodeId,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
        match node {
            NativeView::Node(node) => {
                let callbacks: Vec<SetListener> = node.callbacks.iter()
                    .map(|c| SetListener::from(c))
                    .collect();

                let node = node.node;
                let attributes = node.attributes.clone();
                Self {
                    parent,
                    node_id: node.id,
                    native_name: node.native_name.clone(),
                    text: None,
                    callbacks,
                    attributes,
                }
            }
            NativeView::Text(text) => {
                Self {
                    parent,
                    node_id: text.id,
                    native_name: "".into(),
                    text: Some(text.text.clone()),
                    callbacks: Vec::with_capacity(0),
                    attributes: HashMap::with_capacity(0),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
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