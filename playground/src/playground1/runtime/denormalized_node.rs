use std::borrow::Cow;
use std::collections::HashMap;

use crate::playground1::attribute::Attribute;
use crate::playground1::callback::{CallbackId, CallbackWrapper};
use crate::playground1::local_component::LocalComponentWrapper;
use crate::playground1::node::{Node, NodeChildren, NodeComponentWrapper, NodeId};

pub struct NodeContainer {
    pub nodes: HashMap<NodeId, StrippedNode>,
    pub callbacks: HashMap<CallbackId, CallbackWrapper>,
    pub root_node: NodeId,
}

pub struct StrippedNode {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub native_name: Option<Cow<'static, str>>,
    pub component: NodeComponentWrapper,
    pub text_child: Option<Cow<'static, str>>,
    pub callbacks: Vec<CallbackId>,
    pub children: Vec<NodeId>,
    pub attributes: HashMap<Cow<'static, str>, Attribute>,
}


impl NodeContainer {
    pub fn from_root(node: Node) -> NodeContainer {
        let mut container = NodeContainer {
            nodes: HashMap::new(),
            callbacks: HashMap::new(),
            root_node: node.id,
        };
        container.add_node(node, None);
        container
    }

    fn add_node(&mut self, node: Node, parent: Option<NodeId>) {
        let Node {
            id,
            native_name,
            component,
            children,
            callbacks,
            attributes,
        } = node;
        let (text_child, children) = match children {
            NodeChildren::Empty => (None, Vec::with_capacity(0)),
            NodeChildren::Text(text) => (Some(text), Vec::with_capacity(0)),
            NodeChildren::Nodes(vec) => (None, vec),
        };
        let mut callback_ids = Vec::with_capacity(callbacks.len());
        callbacks.into_iter().for_each(|c| {
            callback_ids.push(c.id);
            self.callbacks.insert(c.id, c);
        });
        let children_ids = children.iter().map(|c| c.id).collect();

        let node = StrippedNode {
            native_name,
            parent,
            id,
            component,
            callbacks: callback_ids,
            text_child,
            children: children_ids,
            attributes,
        };
        self.nodes.insert(id, node);
        children.into_iter().enumerate().for_each(|(pos, c)| {
            self.add_node(c, Some(id))
        });
    }

    pub(crate) fn detach(&mut self, node_id: &NodeId) -> Option<DetachedNode> {
        let option = self.nodes.remove(node_id);
        option.and_then(|n| {
            let mut vec = Vec::new();
            n.callbacks.iter().for_each(|callback_id| {
                if let Some(c) = self.callbacks.remove(callback_id) {
                    vec.push(c);
                }
            });
            Some(DetachedNode {
                node: n,
                callbacks: vec,
            })
        })
    }
    pub(crate) fn swap_node_component(&mut self, id: &NodeId, state: Box<dyn LocalComponentWrapper>) {
        if let Some(n) = self.nodes.get_mut(id) {
            n.component = NodeComponentWrapper::Local(state);
        }
    }
    pub(crate) fn replace_node(&mut self, mut new_node: Node, old_node_id: NodeId) {
        if let Some(old_node) = self.nodes.remove(&old_node_id) {
            let parent = old_node.parent;
            self.replace_child_in_parent(&old_node_id, new_node.id, &parent);
            self.remove_recursive(old_node);
            self.add_node(new_node, parent);
        }
    }

    fn remove_recursive(&mut self, node: StrippedNode) {
        node.callbacks.iter().for_each(|cid| {
            self.callbacks.remove(cid);
        });
        if let Some(parent) = node.parent {
            if let Some(parent) = self.nodes.get_mut(&parent) {
                if let Some(index) = parent.children.iter().position(|cid| cid == &node.id) {
                    parent.children.remove(index);
                }
            }
        }

        node.children.iter().for_each(|c| {
            if let Some(removed) = self.nodes.remove(c) {
                self.remove_recursive(removed);
            }
        })
    }

    fn replace_child_in_parent(&mut self, old_node_id: &NodeId, new_node_id: NodeId, parent: &Option<NodeId>) -> () {
        if let Some(parent_pos) = parent {
            if let Some(parent) = self.nodes.get_mut(parent_pos) {
                if let Some(index) = parent.children.iter().position(|cid| cid == old_node_id) {
                    std::mem::replace(&mut parent.children[index], new_node_id);
                }
            }
        }
    }
    pub fn get_node(&self, node_id: &NodeId) -> Option<&StrippedNode> {
        self.nodes.get(node_id)
    }
}

pub struct DetachedNode {
    pub  node: StrippedNode,
    pub  callbacks: Vec<CallbackWrapper>,
}
