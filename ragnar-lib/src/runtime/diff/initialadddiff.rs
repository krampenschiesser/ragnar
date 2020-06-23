use crate::runtime::diff::operations::{AddNode, DiffOperation, ParentPosition};
use crate::runtime::node_container::NativeView;

pub struct InitialAddDiff<'a> {
    container: &'a NativeView<'a>,
}

impl<'a> InitialAddDiff<'a> {
    pub fn new(container: &'a NativeView<'a>) -> Self {
        Self { container }
    }
    pub fn get_diff(&self) -> Vec<DiffOperation> {
        let mut diff_ops = Vec::new();

        self.add_node(self.container, None, &mut diff_ops);
        diff_ops
    }

    fn add_node(
        &self,
        native_view: &NativeView,
        parent: Option<ParentPosition>,
        diff_ops: &mut Vec<DiffOperation>,
    ) {
        diff_ops.push(DiffOperation::AddNode(AddNode::new(native_view, parent)));
        native_view
            .get_children()
            .iter()
            .enumerate()
            .for_each(|(i, c)| {
                self.add_node(
                    c,
                    Some(ParentPosition {
                        parent: *native_view.get_id(),
                        index: i as u64,
                    }),
                    diff_ops,
                )
            });
    }
}
