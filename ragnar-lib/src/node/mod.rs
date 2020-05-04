use std::ops::Deref;


pub use crate::node::app_node::AppNode;
use crate::node::app_node::UntypedAppNode;
pub use crate::node::local_node::LocalNode;
pub use crate::node::native_node::NativeNode;
pub use crate::node::text_node::TextNode;


pub mod app_node;
pub mod native_node;
pub mod local_node;
pub mod text_node;
pub mod extend_node_children;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash,serde::Serialize,serde::Deserialize)]
pub struct NodeId(pub u64);

impl Deref for NodeId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub enum Node {
    Native(NativeNode),
    Local(LocalNode),
    App(UntypedAppNode),
    Text(TextNode),
}

impl Node {
    pub fn get_id(&self) -> NodeId {
        match self {
            Node::Native(n) => n.id,
            Node::Local(n) => n.id,
            Node::App(n) => n.id,
            Node::Text(n) => n.id,
        }
    }
}
