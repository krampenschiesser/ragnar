use crate::playground1::node::NodeId;
use crate::playground1::callback::CallbackId;

pub mod stripped_app_node;
pub mod stripped_local_node;
pub mod stripped_native_node;

pub use stripped_app_node::StrippedAppNode;
pub use stripped_local_node::StrippedLocalNode;
pub use stripped_native_node::StrippedNativeNode;
use downcast_rs::{Downcast,impl_downcast};


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
            std::mem::replace(&mut children[index], new_node_id);
            Some(index)
        }else {
            None
        }
    }
}
impl_downcast!(StrippedNode);