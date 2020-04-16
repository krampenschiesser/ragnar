use std::collections::HashMap;
use std::rc::Rc;


use crate::callback::{AppCallbackWrapper, CallbackId, LocalCallbackWrapper, NativeCallbackWrapper};
use crate::local_component::LocalComponentWrapper;
use crate::node::{Node, NodeId, TextNode};
use crate::node::app_node::{Converter, UntypedAppNode};
use crate::node::local_node::LocalNode;
use crate::node::native_node::NativeNode;

use crate::runtime::diff::operations::ParentPosition;
use crate::runtime::node_container::stripped_node::{StrippedAppNode, StrippedLocalNode, StrippedNativeNode, StrippedNode};
use std::borrow::Cow;
use crate::Attribute;

pub mod stripped_node;

pub struct NodeContainer {
    pub app_nodes: HashMap<NodeId, StrippedAppNode>,
    pub local_nodes: HashMap<NodeId, StrippedLocalNode>,
    pub native_nodes: HashMap<NodeId, StrippedNativeNode>,
    pub text_nodes: HashMap<NodeId, TextNode>,

    pub local_callbacks: HashMap<CallbackId, LocalCallbackWrapper>,
    pub native_callbacks: HashMap<CallbackId, NativeCallbackWrapper>,
    pub app_callbacks: HashMap<CallbackId, AppCallbackWrapper>,

    pub root_node: NodeId,
}


impl NodeContainer {
    pub fn new(root: NodeId) -> Self {
        NodeContainer {
            app_nodes: HashMap::new(),
            local_nodes: HashMap::new(),
            native_nodes: HashMap::new(),
            text_nodes: HashMap::new(),
            local_callbacks: HashMap::new(),
            native_callbacks: HashMap::new(),
            app_callbacks: HashMap::new(),
            root_node: root,
        }
    }
    pub fn from_root(node: UntypedAppNode) -> NodeContainer {
        let mut container = NodeContainer {
            app_nodes: HashMap::new(),
            local_nodes: HashMap::new(),
            native_nodes: HashMap::new(),
            text_nodes: HashMap::new(),
            app_callbacks: HashMap::new(),
            local_callbacks: HashMap::new(),
            native_callbacks: HashMap::new(),
            root_node: node.id,
        };
        container.root_node = node.id;
        container.add_app_node(node, None, None);
        container
    }

    pub fn native_view<'a>(&'a self, node_id: Option<NodeId>) -> Vec<NativeView<'a>> {
        self.get_native_nodes(&node_id.unwrap_or(self.root_node))
    }

    fn get_native_nodes<'a, 'b>(&'a self, node_id: &'b NodeId) -> Vec<NativeView<'a>> {
        if let Some(node) = self.app_nodes.get(&node_id) {
            node.children.iter().flat_map(|c| self.get_native_nodes(c).into_iter()).collect()
        } else if let Some(node) = self.local_nodes.get(&node_id) {
            node.children.iter().flat_map(|c| self.get_native_nodes(c).into_iter()).collect()
        } else if let Some(node) = self.native_nodes.get(&node_id) {
            let views: Vec<_> = node.children.iter().flat_map(|c| self.get_native_nodes(c).into_iter()).collect();
            let callbacks = node.callbacks.iter().filter_map(|cid| self.native_callbacks.get(cid)).collect();
            let view = NativeNodeView {
                node,
                callbacks,
                children: views,
            };
            vec![NativeView::Node(view)]
        } else {
            Vec::with_capacity(0)
        }
    }

    fn add_app_node(&mut self, node: UntypedAppNode, parent: Option<NodeId>, converters: Option<Vec<Rc<Converter>>>) {
        let (node, callbacks, children) = node.into_stripped(parent, converters);
        callbacks.into_iter().for_each(|c| {
            self.app_callbacks.insert(c.id, c);
        });
        children.into_iter().for_each(|c| {
            self.add_node(c, Some(node.id), node.converters.clone());
        });
        self.app_nodes.insert(node.id, node);
    }

    fn add_local_node(&mut self, node: LocalNode, parent: Option<NodeId>, converters: Option<Vec<Rc<Converter>>>) {
        let (node, callbacks, children) = node.into_stripped(parent);
        callbacks.into_iter().for_each(|c| {
            self.local_callbacks.insert(c.id, c);
        });
        children.into_iter().for_each(|c| {
            self.add_node(c, Some(node.id), converters.clone());
        });
        self.local_nodes.insert(node.id, node);
    }

    fn add_native_node(&mut self, node: NativeNode, parent: Option<NodeId>, converters: Option<Vec<Rc<Converter>>>) {
        let (node, callbacks, children) = node.into_stripped(parent);
        callbacks.into_iter().for_each(|c| {
            self.native_callbacks.insert(c.id, c);
        });
        children.into_iter().for_each(|c| {
            self.add_node(c, Some(node.id), converters.clone());
        });
        self.native_nodes.insert(node.id, node);
    }


    fn add_text_node(&mut self, node: TextNode) {
        self.text_nodes.insert(node.id, node);
    }

    fn add_node(&mut self, node: Node, parent: Option<NodeId>, converters: Option<Vec<Rc<Converter>>>) {
        match node {
            Node::Local(node) => self.add_local_node(node, parent, converters),
            Node::Native(node) => self.add_native_node(node, parent, converters),
            Node::App(node) => self.add_app_node(node, parent, converters),
            Node::Text(node) => self.add_text_node(node),
        }
    }

    pub(crate) fn swap_node_component(&mut self, id: &NodeId, state: Box<dyn LocalComponentWrapper>) {
        if let Some(n) = self.local_nodes.get_mut(id) {
            n.component = state;
        }
    }
    pub(crate) fn replace_local_node(&mut self, new_node: LocalNode, old_node_id: NodeId) -> Option<(Option<ParentPosition>, NodeContainer)> {
        if let Some(old_node) = self.local_nodes.remove(&old_node_id) {
            let parent = old_node.parent;
            let index = self.replace_child_in_parent(&old_node_id, new_node.id, &parent);
            let mut container = NodeContainer::new(old_node_id);
            self.remove_local_recursive(old_node, &mut container);
            self.add_node(Node::Local(new_node), parent, None);
            let parent = if let Some((p, i)) = parent.and_then(|p| index.map(|i| (p, i))) {
                Some(ParentPosition { parent: p, index: i as u64 })
            } else {
                None
            };
            Some((parent, container))
        } else {
            None
        }
    }
    fn remove_and_add_callback(&mut self, callback_id: &CallbackId, container: &mut NodeContainer) {
        if let Some(local_callback) = self.local_callbacks.remove(callback_id) {
            container.local_callbacks.insert(*callback_id, local_callback);
        } else if let Some(app_callback) = self.app_callbacks.remove(callback_id) {
            container.app_callbacks.insert(*callback_id, app_callback);
        } else if let Some(native_callback) = self.native_callbacks.remove(callback_id) {
            container.native_callbacks.insert(*callback_id, native_callback);
        }
    }
    fn remove_local_recursive(&mut self, node: StrippedLocalNode, container: &mut NodeContainer) {
        self.remove_callbacks_and_parent(&node, container);
        self.remove_children(node.get_children(), container);
        container.local_nodes.insert(node.get_id(), node);
    }
    fn remove_app_recursive(&mut self, node: StrippedAppNode, container: &mut NodeContainer) {
        self.remove_callbacks_and_parent(&node, container);
        self.remove_children(node.get_children(), container);
        container.app_nodes.insert(node.get_id(), node);
    }
    fn remove_native_recursive(&mut self, node: StrippedNativeNode, container: &mut NodeContainer) {
        self.remove_callbacks_and_parent(&node, container);
        self.remove_children(node.get_children(), container);
        container.native_nodes.insert(node.get_id(), node);
    }

    fn remove_children(&mut self, children: &[NodeId], container: &mut NodeContainer) {
        children.iter().for_each(|c| {
            if let Some(removed) = self.local_nodes.remove(c) {
                self.remove_local_recursive(removed, container);
            } else if let Some(removed) = self.app_nodes.remove(c) {
                self.remove_app_recursive(removed, container);
            } else if let Some(removed) = self.native_nodes.remove(c) {
                self.remove_native_recursive(removed, container);
            }
        });
    }

    fn remove_callbacks_and_parent<T: StrippedNode>(&mut self, node: &T, container: &mut NodeContainer) {
        node.get_callbacks().iter().for_each(|cid| {
            self.remove_and_add_callback(cid, container);
        });
        if let Some(parent) = node.get_parent() {
            if let Some(parent) = self.get_node_mut(&parent) {
                if let Some(index) = parent.get_children().iter().position(|cid| cid == &node.get_id()) {
                    parent.get_children_mut().remove(index);
                }
            }
        }
    }

    fn replace_child_in_parent(&mut self, old_node_id: &NodeId, new_node_id: NodeId, parent: &Option<NodeId>) -> Option<usize> {
        if let Some(parent_pos) = parent {
            if let Some(parent) = self.get_node_mut(parent_pos) {
                return parent.replace_child(old_node_id, new_node_id);
            }
        }
        None
    }
    pub fn get_node(&self, node_id: &NodeId) -> Option<&dyn StrippedNode> {
        if let Some(node) = self.native_nodes.get(node_id) {
            return Some(node);
        }
        if let Some(node) = self.local_nodes.get(node_id) {
            return Some(node);
        }
        if let Some(node) = self.app_nodes.get(node_id) {
            return Some(node);
        }
        return None;
    }
    pub fn get_node_mut(&mut self, node_id: &NodeId) -> Option<&mut dyn StrippedNode> {
        if let Some(node) = self.native_nodes.get_mut(node_id) {
            return Some(node);
        }
        if let Some(node) = self.local_nodes.get_mut(node_id) {
            return Some(node);
        }
        if let Some(node) = self.app_nodes.get_mut(node_id) {
            return Some(node);
        }
        return None;
    }
}

pub enum NativeView<'a> {
    Node(NativeNodeView<'a>),
    Text(TextNode),
}

pub struct NativeNodeView<'a> {
    pub node: &'a StrippedNativeNode,
    pub callbacks: Vec<&'a NativeCallbackWrapper>,
    pub children: Vec<NativeView<'a>>,
}

impl<'a> NativeView<'a> {
    pub fn get_id(&self) -> &NodeId {
        match self {
            NativeView::Text(t) => &t.id,
            NativeView::Node(n) => &n.node.id,
        }
    }

    pub fn get_native_name(&self) -> &str {
        match self {
            NativeView::Text(t) => "",
            NativeView::Node(n) => &n.node.native_name,
        }
    }

    pub fn get_children(&self) -> &[NativeView<'a>] {
        match self {
            NativeView::Text(t) => &[],
            NativeView::Node(n) => &n.children,
        }
    }

    pub fn get_attributes(&self) -> &HashMap<Cow<'static, str>, Attribute> {
        match self {
            NativeView::Text(t) => &EMPTY_ATTRIBUTES,
            NativeView::Node(n) => &n.node.attributes,
        }
    }
    pub fn get_callbacks(&self) -> &[&'a NativeCallbackWrapper] {
        match self {
            NativeView::Text(t) => &[],
            NativeView::Node(n) => &n.callbacks,
        }
    }
}

lazy_static! {
    static ref EMPTY_ATTRIBUTES: HashMap<Cow<'static, str>, Attribute> = HashMap::with_capacity(0);
}
