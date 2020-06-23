use crate::callback::CallbackId;
use crate::node::NodeId;

pub mod stripped_app_node;
pub mod stripped_local_node;
pub mod stripped_native_node;

use downcast_rs::{impl_downcast, Downcast};
pub use stripped_app_node::StrippedAppNode;
pub use stripped_local_node::StrippedLocalNode;
pub use stripped_native_node::StrippedNativeNode;

pub trait StrippedNode: Downcast {
    fn get_id(&self) -> NodeId;
    fn get_parent(&self) -> Option<NodeId>;
    fn has_parent(&self) -> bool;
    fn get_children(&self) -> &[NodeId];
    fn get_children_mut(&mut self) -> &mut Vec<NodeId>;
    fn get_callbacks(&self) -> &[CallbackId];

    fn replace_child(&mut self, old_node_id: &NodeId, new_node_id: NodeId) -> Option<usize> {
        let children = self.get_children_mut();
        if let Some(index) = children.iter().position(|cid| cid == old_node_id) {
            let _ = std::mem::replace(&mut children[index], new_node_id);
            Some(index)
        } else {
            None
        }
    }
}
impl_downcast!(StrippedNode);
