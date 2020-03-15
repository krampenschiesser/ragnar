use crate::playground1::runtime::denormalized_node::NodeContainer;
use crate::playground1::runtime::diff::operations::DiffOperation;

pub mod operations;

pub struct CompleteDiff<'original, 'new> {
    original: &'original NodeContainer,
    new: &'new NodeContainer,
}

impl<'original, 'new> CompleteDiff<'original, 'new> {
    pub fn new(original: &'original NodeContainer, new: &'new NodeContainer) -> Self {
        Self {
            original,
            new,
        }
    }

    pub fn diff(&self) -> Vec<DiffOperation<'new>> {
        vec![]
    }
}