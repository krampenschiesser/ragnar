use std::borrow::Cow;
use std::collections::HashMap;

use crate::attribute::Attribute;
use crate::callback::{CallbackId, NativeCallbackWrapper};
use crate::node::{Node, NodeId};
use crate::node::native_node::{NativeNode};
use crate::runtime::node_container::stripped_node::StrippedNode;

pub struct StrippedNativeNode {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub native_name: Cow<'static, str>,
    pub callbacks: Vec<CallbackId>,
    pub children: Vec<NodeId>,
    pub attributes: HashMap<Cow<'static, str>, Attribute>,
}

impl NativeNode {
    pub fn into_stripped(self, parent: Option<NodeId>) -> (StrippedNativeNode, Vec<NativeCallbackWrapper>, Vec<Node>) {
        let NativeNode {
            id,
            native_name,
            children,
            callbacks,
            attributes,
        } = self;
        let (child_ids, children) = (children.iter().map(|n| n.get_id()).collect(), children);

        let node = StrippedNativeNode {
            callbacks: callbacks.iter().map(|c| c.id).collect(),
            id,
            children: child_ids,
            attributes,
            native_name,
            parent,
        };
        (node, callbacks, children)
    }
}

impl StrippedNode for StrippedNativeNode {
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