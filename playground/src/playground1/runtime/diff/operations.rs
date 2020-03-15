use std::borrow::Cow;

use crate::playground1::attribute::Attribute;
use crate::playground1::callback::{CallbackId, CallbackWrapper};
use crate::playground1::node::NodeId;
use crate::playground1::runtime::denormalized_node::StrippedNode;

pub struct SetAttribute<'a> {
    node_id: NodeId,
    attribute_name: &'a str,
    attribute_value: &'a Attribute,
}

pub struct RemoveAttribute {
    node_id: NodeId,
    attribute_name: Cow<'static, str>,
}

pub struct RemoveListener {
    node_id: NodeId,
    callback_id: CallbackId,
}

pub struct SetListener<'a> {
    callback: &'a CallbackWrapper
}

pub struct RemoveNode {
    node_id: NodeId
}

pub struct AddNode<'a> {
    node: &'a StrippedNode,
    callbacks: Vec<&'a CallbackWrapper>,
}

pub struct SwapNode<'a> {
    original: NodeId,
    node: &'a StrippedNode,
    callbacks: Vec<&'a CallbackWrapper>,
}

pub enum DiffOperation<'a> {
    SetAttribute(SetAttribute<'a>),
    RemoveAttribute(RemoveAttribute),

    SetListener(SetListener<'a>),
    RemoveListener(RemoveListener),

    RemoveNode(RemoveNode),
    AddNode(AddNode<'a>),
    SwapNode(SwapNode<'a>),
}