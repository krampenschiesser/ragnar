use std::rc::Rc;

use crate::playground1::app_component::AppEvent;
use crate::playground1::callback::{AppCallbackWrapper, CallbackId};
use crate::playground1::node::{Node, NodeId};
use crate::playground1::node::app_node::{AppNode, Converter, UntypedAppNode};
use crate::playground1::runtime::node_container::stripped_node::StrippedNode;

pub struct StrippedAppNode {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub callbacks: Vec<CallbackId>,
    pub converters: Option<Vec<Rc<Converter>>>,
}

impl UntypedAppNode {
    pub fn into_stripped(self, parent: Option<NodeId>, converters: Option<Vec<Rc<Converter>>>) -> (StrippedAppNode, Vec<AppCallbackWrapper>, Vec<Node>) {
        let UntypedAppNode {
            id,
            children,
            callbacks,
            converter,
        } = self;

        let converters = if let Some(mut converters) = converters {
            if let Some(converter) = converter {
                converters.push(Rc::new(converter));
            }
            Some(converters)
        } else if let Some(converter) = converter {
            Some(vec![Rc::new(converter)])
        } else {
            None
        };

        let children = children.unwrap_or(Vec::with_capacity(0));
        let child_ids = children.iter().map(|c| c.get_id()).collect();

        let node = StrippedAppNode {
            callbacks: callbacks.iter().map(|c| c.id).collect(),
            id,
            children: child_ids,
            parent,
            converters,
        };
        (node, callbacks, children)
    }
}

impl StrippedNode for StrippedAppNode {
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

    fn get_children_mut(&mut self) -> &mut Vec<NodeId> {
        self.children.as_mut()
    }

    fn get_callbacks(&self) -> &[CallbackId] {
        self.callbacks.as_slice()
    }
}