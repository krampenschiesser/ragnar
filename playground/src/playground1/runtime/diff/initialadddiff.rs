use crate::playground1::node::NodeId;
use crate::playground1::runtime::denormalized_node::NodeContainer;
use crate::playground1::runtime::diff::DiffError;
use crate::playground1::runtime::diff::operations::{AddNode, DiffOperation, ParentPosition};

pub struct InitialAddDiff<'a> {
    container: &'a NodeContainer,
}

impl<'a> InitialAddDiff<'a> {
    pub fn get_diff(&self) -> Result<Vec<DiffOperation>, DiffError> {
        let mut diff_ops = Vec::new();

        let root_id = self.container.root_node;
        self.add_node(root_id, None, &mut diff_ops)?;
        Ok(diff_ops)
    }

    fn add_node(&self, id: NodeId, parent: Option<ParentPosition>, diff_ops: &mut Vec<DiffOperation>) -> Result<(), DiffError> {
        let node = self.container.get_node(&id).ok_or(DiffError::NewNodeNotFound(id))?;
        diff_ops.push(DiffOperation::AddNode(AddNode::new(node, self.container, parent)?));
        let option = node.children.iter().enumerate().map(|(i, c)| {
            self.add_node(*c, Some(ParentPosition { parent: id, index: i as u64 }), diff_ops)
        }).filter(|e| e.is_err()).next();
        if let Some(res) = option {
            res
        } else {
            Ok(())
        }
    }
}