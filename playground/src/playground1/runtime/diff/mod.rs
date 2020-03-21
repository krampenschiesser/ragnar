use std::borrow::Cow;

use crate::playground1::callback::CallbackId;
use crate::playground1::node::NodeId;
use crate::playground1::runtime::diff::operations::{AddNode, DiffOperation, ParentPosition, RemoveAttribute, RemoveListener, RemoveNode, SetAttribute, SetListener, SwapNode, SwapNodeId};
use crate::playground1::runtime::node_container::{NativeView, NodeContainer};
use crate::playground1::runtime::node_container::stripped_node::StrippedNode;

pub mod operations;
pub mod initialadddiff;

pub struct CompleteDiff<'a> {
    original: &'a Vec<NativeView<'a>>,
    new: &'a Vec<NativeView<'a>>,
}

pub enum DiffError {
    NonNative(NodeId),
    OldNodeNotFound(NodeId),
    NewNodeNotFound(NodeId),
    OldCallbackNotFound(CallbackId),
    NewCallbackNotFound(CallbackId),
}

impl<'a> CompleteDiff<'a> {
    pub fn new(original: &'a Vec<NativeView<'a>>, new: &'a Vec<NativeView<'a>>) -> Self {
        Self {
            original,
            new,
        }
    }

    pub fn diff(&self) -> Vec<DiffOperation> {
        let mut diff_ops = Vec::new();
        self.diff_children(self.original.as_slice(),self.new.as_slice(), None, &mut diff_ops);
        diff_ops
    }

    fn diff_nodes(&self, original: &'a NativeView, new: &'a NativeView, diff_ops: &mut Vec<DiffOperation>, parent: Option<ParentPosition>) {
        if original.node.id == new.node.id {
            self.diff_children(original.children.as_slice(), new.children.as_slice(), parent, diff_ops);
        } else if original.node.native_name != new.node.native_name {
            diff_ops.push(DiffOperation::SwapNode(SwapNode::new(original.node.id, new, parent)));
            self.diff_children(&original.children, &new.children, parent, diff_ops);
        } else {
            diff_ops.push(DiffOperation::SwapNodeId(SwapNodeId { original: original.node.id, new: new.node.id }));
            self.diff_attributes(original, new, diff_ops);
            self.diff_callbacks(original, new, diff_ops);
            self.diff_children(&original.children, &new.children, parent, diff_ops);
        }
    }

    fn diff_attributes(&self, original: &'a NativeView, new: &'a NativeView, diff_ops: &mut Vec<DiffOperation>) {
        let all_keys = original.node.attributes.keys().chain(new.node.attributes.keys());
        for key in all_keys {
            let original_value = original.node.attributes.get(key);
            let new_value = new.node.attributes.get(key);
            if original_value != new_value {
                if let Some(new_value) = new_value {
                    diff_ops.push(DiffOperation::SetAttribute(SetAttribute { node_id: new.node.id, attribute_name: key.clone(), attribute_value: new_value.clone() }));
                } else {
                    diff_ops.push(DiffOperation::RemoveAttribute(RemoveAttribute { node_id: new.node.id, attribute_name: key.clone() }));
                }
            }
        }
    }

    fn diff_children(&self, original: &[NativeView], new: &[NativeView], parent: Option<ParentPosition>, diff_ops: &mut Vec<DiffOperation>) {
        let mut iter_original = original.iter();
        let mut iter_new = new.iter();

        loop {
            let original_child = iter_original.next();
            let new_child = iter_new.next();
            if new_child.is_none() && original_child.is_none() {
                break;
            }
            if let Some(original_child) = original_child {
                if let Some(new_child) = new_child {
                    self.diff_nodes(original_child, new_child, diff_ops, parent);
                } else {
                    diff_ops.push(DiffOperation::RemoveNode(RemoveNode { node_id: original_child.node.id }))
                }
            } else if let Some(new_child) = new_child {
                diff_ops.push(DiffOperation::AddNode(AddNode::new(new_child, parent)));
            }
        }
    }
    fn diff_callbacks(&self, original: &'a NativeView, new: &'a NativeView, diff_ops: &mut Vec<DiffOperation>) {
        let original_callbacks = &original.callbacks;
        let new_callbacks = &new.callbacks;
        let new_callback_names: Vec<_> = new_callbacks.iter().map(|c| &c.native_name).collect();


        original_callbacks.iter().filter(|n| !new_callback_names.contains(&&n.native_name)).for_each(|n| {
            diff_ops.push(DiffOperation::RemoveListener(RemoveListener { node_id: n.node_id, callback_id: n.id }))
        });
        new_callbacks.iter().for_each(|n| {
            diff_ops.push(DiffOperation::SetListener(SetListener::from(n)))
        });
    }
}