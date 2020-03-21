use std::borrow::Cow;
use std::collections::HashMap;

use crate::playground1::attribute::Attribute;
use crate::playground1::callback::NativeCallbackWrapper;
use crate::playground1::INCREMENTER;

use crate::playground1::node::{Node,  NodeId};

pub struct NativeNode {
    pub id: NodeId,
    pub native_name: Cow<'static, str>,
    pub children: NodeChildren,
    pub callbacks: Vec<NativeCallbackWrapper>,
    pub attributes: HashMap<Cow<'static, str>, Attribute>,
}

pub enum NodeChildren {
    Empty,
    Text(Cow<'static, str>),
    Nodes(Vec<Node>),
}

impl NativeNode {
    pub fn new<T: Into<Cow<'static, str>>>(name: T) -> Self {
        NativeNode {
            id: NodeId(INCREMENTER.get_next()),
            native_name: name.into(),
            children: NodeChildren::Empty,
            callbacks: Vec::new(),
            attributes: HashMap::new(),
        }
    }

    pub fn set<T: Into<Cow<'static, str>>, A: Into<Attribute>>(mut self, name: T, attribute: A) -> Self {
        self.attributes.insert(name.into(), attribute.into());
        self
    }

    pub fn with_callback<T: Into<NativeCallbackWrapper>>(mut self, t: T) -> Self {
        let mut callback_wrapper = t.into();
        callback_wrapper.node_id = self.id;
        self.callbacks.push(callback_wrapper);
        self
    }
    pub fn with_text<T: Into<Cow<'static, str>>>(mut self, t: T) -> Self {
        self.children = NodeChildren::Text(t.into());
        self
    }
    pub fn with_children<T: Into<Node>>(mut self, nodes: Vec<T>) -> Self {
        for node in nodes {
            self = self.with_child(node);
        }
        self
    }

    pub fn with_child<T: Into<Node>>(mut self, node: T) -> Self {
        let needs_new_assignment = match &self.children {
            NodeChildren::Text(_) => true,
            NodeChildren::Empty => true,
            NodeChildren::Nodes(_) => false
        };
        if needs_new_assignment {
            self.children = NodeChildren::Nodes(vec![node.into()]);
        } else {}
        self
    }
}


impl Into<Node> for NativeNode {
    fn into(self) -> Node {
        Node::Native(self)
    }
}