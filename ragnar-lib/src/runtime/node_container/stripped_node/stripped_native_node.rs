use std::borrow::Cow;
use std::collections::HashMap;

use crate::attribute::Attribute;
use crate::callback::{CallbackId, NativeCallbackWrapper};
use crate::node::{Node, NodeId};
use crate::node::native_node::{NativeNode, NodeChildren};
use crate::runtime::node_container::stripped_node::StrippedNode;

pub struct StrippedNativeNode {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub native_name: Cow<'static, str>,
    pub text_child: Option<Cow<'static, str>>,
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
        let (text, child_ids, children) = match children {
            NodeChildren::Nodes(nodes) => {
                (None, nodes.iter().map(|n| n.get_id()).collect(), nodes)
            }
            NodeChildren::Text(t) => {
                (Some(t), Vec::with_capacity(0), Vec::with_capacity(0))
            }
            _ => (None, Vec::with_capacity(0), Vec::with_capacity(0))
        };

        let node = StrippedNativeNode {
            callbacks: callbacks.iter().map(|c| c.id).collect(),
            id,
            children: child_ids,
            attributes,
            native_name,
            parent,
            text_child: text,
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