use crate::node::{NodeId, Node};
use crate::callback::{CallbackId, LocalCallbackWrapper};
use crate::local_component::LocalComponentWrapper;
use crate::runtime::node_container::stripped_node::StrippedNode;
use crate::node::local_node::LocalNode;

pub struct  StrippedLocalNode {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub callbacks: Vec<CallbackId>,
    pub component: Box<dyn LocalComponentWrapper>
}
impl LocalNode {
    pub fn into_stripped(self, parent: Option<NodeId>) -> (StrippedLocalNode, Vec<LocalCallbackWrapper>, Vec<Node>) {
        let LocalNode {
            component,
            id,
            children,
            callbacks,
        } = self;
        let children = children.unwrap_or(Vec::with_capacity(0));
        let child_ids = children.iter().map(|c|c.get_id()).collect();

        let node = StrippedLocalNode {
            callbacks: callbacks.iter().map(|c| c.id).collect(),
            id,
            children: child_ids,
            parent,
            component,
        };
        (node, callbacks, children)
    }
}
impl StrippedNode for StrippedLocalNode {
    fn get_id(&self) -> NodeId {
        self.id
    }

    fn get_parent(&self) -> Option<NodeId> {
        self.parent
    }

    fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    fn get_children(&self) -> &[NodeId] {
        self.children.as_slice()
    }

    fn get_callbacks(&self) -> &[CallbackId] {
        self.callbacks.as_slice()
    }
    fn get_children_mut(&mut self) -> &mut Vec<NodeId> {
        self.children.as_mut()
    }
}