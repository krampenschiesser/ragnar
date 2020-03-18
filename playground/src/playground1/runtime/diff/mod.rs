use std::borrow::Cow;

use crate::playground1::callback::CallbackId;
use crate::playground1::node::NodeId;
use crate::playground1::runtime::denormalized_node::{NodeContainer, StrippedNode};
use crate::playground1::runtime::diff::operations::{AddNode, DiffOperation, ParentPosition, RemoveAttribute, RemoveListener, RemoveNode, SetAttribute, SetListener, SwapNode, SwapNodeId};

pub mod operations;
pub mod initialadddiff;
pub struct CompleteDiff<'a> {
    original: &'a NodeContainer,
    new: &'a NodeContainer,
}

pub struct PartialDiff<'a> {
    original: &'a NodeContainer,
}

pub enum DiffError {
    NonNative(NodeId),
    OldNodeNotFound(NodeId),
    NewNodeNotFound(NodeId),
    OldCallbackNotFound(CallbackId),
    NewCallbackNotFound(CallbackId),
}

impl<'a> CompleteDiff<'a> {
    pub fn new(original: &'a NodeContainer, new: &'a NodeContainer) -> Self {
        Self {
            original,
            new,
        }
    }

    pub fn diff_partial(&self, old: NodeId, new: NodeId, parent: Option<ParentPosition>) -> Result<Vec<DiffOperation>, DiffError> {
        let mut diff_ops = Vec::new();

        let new_node = self.new.get_node(&new).ok_or(DiffError::NewNodeNotFound(new))?;
        let old_node = self.original.get_node(&old).ok_or(DiffError::OldNodeNotFound(new))?;
        self.diff_nodes(old_node, new_node, &mut diff_ops, parent)?;
        Ok(diff_ops)
    }

    pub fn diff(&self) -> Result<Vec<DiffOperation>, DiffError> {
        let new_root = self.new.root_node;
        let old_root = self.original.root_node;
        self.diff_partial(old_root, new_root, None)
    }

    fn diff_nodes(&self, original: &'a StrippedNode, new: &'a StrippedNode, diff_ops: &mut Vec<DiffOperation>, parent: Option<ParentPosition>) -> Result<(), DiffError> {
        if original.id == new.id {
            self.diff_children(original, new, diff_ops)?;
        } else if original.native_name != new.native_name {
            if original.native_name.is_some() && new.native_name.is_some() {
                diff_ops.push(DiffOperation::SwapNode(SwapNode::new(original.id, new, self.new, parent)?));
                self.diff_children(original, new, diff_ops)?;
            } else if original.native_name.is_none() {
                diff_ops.push(DiffOperation::AddNode(AddNode::new(new, self.new, parent)?));
                self.add_children(new, diff_ops)?;
            } else if new.native_name.is_none() {
                diff_ops.push(DiffOperation::RemoveNode(RemoveNode { node_id: original.id }));
                return Ok(());
            }
        } else if original.native_name == new.native_name && new.native_name.is_some() {
            diff_ops.push(DiffOperation::SwapNodeId(SwapNodeId { original: original.id, new: new.id }));
            self.diff_attributes(original, new, diff_ops);
            self.diff_callbacks(original, new, diff_ops)?;
            self.diff_children(original, new, diff_ops)?;
        }
        Ok(())
    }

    fn add_children(&self, new: &StrippedNode, diff_ops: &mut Vec<DiffOperation>) -> Result<(), DiffError> {
        for (index, child_id) in new.children.iter().enumerate() {
            let child_node = self.new.get_node(child_id).ok_or(DiffError::NewNodeNotFound(*child_id))?;
            let parent = ParentPosition { parent: new.id, index: index as u64 };
            diff_ops.push(DiffOperation::AddNode(AddNode::new(child_node, self.new, Some(parent))?));
            self.add_children(child_node, diff_ops)?;
        }
        Ok(())
    }

    fn diff_children(&self, original: &'a StrippedNode, new: &'a StrippedNode, diff_ops: &mut Vec<DiffOperation>) -> Result<(), DiffError> {
        let combined = original.children.iter().zip(new.children.iter()).enumerate();
        for ((index, (original_id, new_id))) in combined {
            let original_node = self.original.get_node(original_id).ok_or(DiffError::OldNodeNotFound(*original_id))?;
            let new_node = self.new.get_node(original_id).ok_or(DiffError::NewNodeNotFound(*original_id))?;

            let parent = ParentPosition { parent: new.id, index: index as u64 };
            self.diff_nodes(original_node, new_node, diff_ops, Some(parent))?;
        }
        Ok(())
    }

    fn diff_attributes(&self, original: &'a StrippedNode, new: &'a StrippedNode, diff_ops: &mut Vec<DiffOperation>) {
        let all_keys = original.attributes.keys().chain(new.attributes.keys());
        for key in all_keys {
            let original_value = original.attributes.get(key);
            let new_value = new.attributes.get(key);
            if original_value != new_value {
                if let Some(new_value) = new_value {
                    diff_ops.push(DiffOperation::SetAttribute(SetAttribute { node_id: new.id, attribute_name: key.clone(), attribute_value: new_value.clone() }));
                } else {
                    diff_ops.push(DiffOperation::RemoveAttribute(RemoveAttribute { node_id: new.id, attribute_name: key.clone() }));
                }
            }
        }
    }
    fn diff_callbacks(&self, original: &'a StrippedNode, new: &'a StrippedNode, diff_ops: &mut Vec<DiffOperation>) -> Result<(), DiffError> {
        let original_callbacks: Vec<_> = original.callbacks.iter()
            .filter_map(|id| self.original.get_callback(id))
            .filter(|c| c.callback_type.is_native())
            .collect();
        let new_callbacks: Vec<_> = new.callbacks.iter()
            .filter_map(|id| self.new.get_callback(id))
            .filter(|c| c.callback_type.is_native())
            .collect();

        let new_callback_names: Vec<_> = new_callbacks.iter().filter_map(|c| c.callback_type.get_native_name()).collect();

        let unwrap_msg = "At this time the callback is native";

        original_callbacks.iter().filter(|n| !new_callback_names.contains(&n.callback_type.get_native_name().expect(unwrap_msg))).for_each(|n| {
            diff_ops.push(DiffOperation::RemoveListener(RemoveListener { node_id: n.node_id, callback_id: n.id }))
        });
        new_callbacks.iter().for_each(|n| {
            diff_ops.push(DiffOperation::SetListener(SetListener::may_from(n).expect(unwrap_msg)))
        });

        Ok(())
    }
}