use crate::playground1::node::NodeId;
use crate::playground1::runtime::diff::DiffError;
use crate::playground1::runtime::diff::operations::{AddNode, DiffOperation, ParentPosition};
use crate::playground1::runtime::node_container::{NativeView, NodeContainer};

pub struct InitialAddDiff<'a> {
    container: &'a NativeView<'a>,
}

impl<'a> InitialAddDiff<'a> {
    pub fn get_diff(&self) -> Vec<DiffOperation> {
        let mut diff_ops = Vec::new();

        self.add_node(self.container, None, &mut diff_ops);
        diff_ops
    }

    fn add_node(&self, native_view: &NativeView, parent: Option<ParentPosition>, diff_ops: &mut Vec<DiffOperation>) {
        diff_ops.push(DiffOperation::AddNode(AddNode::new(native_view, parent)));
        native_view.children.iter().enumerate().map(|(i, c)| {
            self.add_node(c, Some(ParentPosition { parent: native_view.node.id, index: i as u64 }), diff_ops)
        });
    }
}