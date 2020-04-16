use crate::callback::CallbackId;
use crate::node::NodeId;
use crate::runtime::diff::operations::{AddNode, DiffOperation, ParentPosition, RemoveAttribute, RemoveListener, RemoveNode, SetAttribute, SetListener, SwapNode, SwapNodeId};
use crate::runtime::node_container::NativeView;


pub mod operations;
pub mod initialadddiff;

pub struct CompleteDiff<'a> {
    original: &'a Vec<NativeView<'a>>,
    new: &'a Vec<NativeView<'a>>,
}

pub enum DiffError {
    HandlerAlreadyUsed(CallbackId,NodeId),
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
        self.diff_children(self.original.as_slice(), self.new.as_slice(), None, &mut diff_ops);
        diff_ops
    }

    fn diff_nodes(&self, original: &'a NativeView, new: &'a NativeView, diff_ops: &mut Vec<DiffOperation>, parent: Option<ParentPosition>) {
        if original.get_id() == new.get_id() {
            self.diff_children(original.get_children(), new.get_children(), parent, diff_ops);
        } else if original.get_native_name() != new.get_native_name() {
            diff_ops.push(DiffOperation::SwapNode(SwapNode::new(*original.get_id(), new, parent)));
            self.diff_children(&original.get_children(), &new.get_children(), parent, diff_ops);
        } else {
            diff_ops.push(DiffOperation::SwapNodeId(SwapNodeId { original: *original.get_id(), new: *new.get_id() }));
            self.diff_attributes(original, new, diff_ops);
            self.diff_callbacks(original, new, diff_ops);
            self.diff_children(&original.get_children(), &new.get_children(), parent, diff_ops);
        }
    }

    fn diff_attributes(&self, original: &'a NativeView, new: &'a NativeView, diff_ops: &mut Vec<DiffOperation>) {
        let all_keys = original.get_attributes().keys().chain(new.get_attributes().keys());
        for key in all_keys {
            let original_value = original.get_attributes().get(key);
            let new_value = new.get_attributes().get(key);
            if original_value != new_value {
                if let Some(new_value) = new_value {
                    diff_ops.push(DiffOperation::SetAttribute(SetAttribute { node_id: *new.get_id(), attribute_name: key.clone(), attribute_value: new_value.clone() }));
                } else {
                    diff_ops.push(DiffOperation::RemoveAttribute(RemoveAttribute { node_id: *new.get_id(), attribute_name: key.clone() }));
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
                    diff_ops.push(DiffOperation::RemoveNode(RemoveNode { node_id: *original_child.get_id() }))
                }
            } else if let Some(new_child) = new_child {
                diff_ops.push(DiffOperation::AddNode(AddNode::new(new_child, parent)));
            }
        }
    }
    fn diff_callbacks(&self, original: &'a NativeView, new: &'a NativeView, diff_ops: &mut Vec<DiffOperation>) {
        let original_callbacks = &original.get_callbacks();
        let new_callbacks = &new.get_callbacks();
        let new_callback_names: Vec<_> = new_callbacks.iter().map(|c| &c.native_name).collect();


        original_callbacks.iter().filter(|n| !new_callback_names.contains(&&n.native_name)).for_each(|n| {
            diff_ops.push(DiffOperation::RemoveListener(RemoveListener { node_id: n.node_id, callback_id: n.id }))
        });
        new_callbacks.iter().for_each(|n| {
            diff_ops.push(DiffOperation::SetListener(SetListener::from(n)))
        });
    }
}