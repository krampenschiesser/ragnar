use crate::playground1::node::{NodeId, Node};
use crate::playground1::local_component::LocalComponentWrapper;
use crate::playground1::callback::LocalCallbackWrapper;
use crate::playground1::INCREMENTER;

pub struct LocalNode {
    pub id: NodeId,
    pub component: Box<dyn LocalComponentWrapper>,
    pub children: Option<Vec<Node>>,
    pub callbacks: Vec<LocalCallbackWrapper>,
}


impl LocalNode {
    pub fn new<C: 'static + LocalComponentWrapper>(component: C) -> Self {
        LocalNode {
            id: NodeId(INCREMENTER.get_next()),
            component: Box::new(component),
            children: None,
            callbacks: Vec::new(),
        }
    }

    pub fn with_callback<T: Into<LocalCallbackWrapper>>(mut self, t: T) -> Self {
        let mut callback_wrapper = t.into();
        callback_wrapper.node_id = self.id;
        self.callbacks.push(callback_wrapper);
        self
    }
    
    pub fn with_children<T: Into<Node>>(mut self, nodes: Vec<T>) -> Self {
        for node in nodes {
            self = self.with_child(node);
        }
        self
    }

    pub fn with_child<T: Into<Node>>(mut self, node: T) -> Self {
        if self.children.is_none() {
            self.children = Some(Vec::with_capacity(1));
        }
        if let Some(children) = &mut self.children {
            children.push(node.into());
        }
        self
    }
}

impl Into<Node> for LocalNode {
    fn into(self) -> Node {
        Node::Local(self)
    }
}