use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Deref;

use crate::playground1::attribute::Attribute;
use crate::playground1::callback::CallbackWrapper;
use crate::playground1::INCREMENTER;
use crate::playground1::local_component::{LocalComponent, LocalComponentWrapper};
use crate::playground1::native_component::{NativeComponent, NativeComponentWrapper};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct NodeId(pub u64);

impl Deref for NodeId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Node {
    pub id: NodeId,
    pub native_name: Option<Cow<'static, str>>,
    pub component: NodeComponentWrapper,
    pub children: NodeChildren,
    pub callbacks: Vec<CallbackWrapper>,
    pub attributes: HashMap<Cow<'static, str>, Attribute>,
}

impl Node {
    pub fn empty() -> Self {
        Node {
            id: NodeId(INCREMENTER.get_next()),
            native_name: None,
            component: NodeComponentWrapper::None,
            children: NodeChildren::Empty,
            callbacks: Vec::new(),
            attributes: HashMap::new(),
        }
    }

    pub fn set<T: Into<Cow<'static, str>>, A: Into<Attribute>>(mut self, name: T, attribute: A) -> Self{
        self.attributes.insert(name.into(), attribute.into());
        self
    }

    pub fn with_callback<T: Into<CallbackWrapper>>(mut self, t: T) -> Self {
        let mut callback_wrapper = t.into();
        callback_wrapper.node_id = self.id;
        self.callbacks.push(callback_wrapper);
        self
    }
    pub fn with_text<T: Into<Cow<'static, str>>>(mut self, t: T) -> Self {
        self.children = NodeChildren::Text(t.into());
        self
    }
    pub fn with_local_component(mut self, t: impl LocalComponentWrapper + 'static) -> Self {
        self.component = NodeComponentWrapper::Local(Box::new(t));
        self
    }
    pub fn with_native_component(mut self, t: impl NativeComponent + 'static) -> Self {
        self.component = NodeComponentWrapper::Native(Box::new(t));
        self
    }
    pub fn with_children(mut self, nodes: Vec<Node>) -> Self {
        for node in nodes {
            self = self.with_child(node);
        }
        self
    }

    pub fn with_child(mut self, node: Node) -> Self {
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
    pub fn with_native_name<T: Into<Cow<'static, str>>>(mut self, t: T) -> Self {
        self.native_name = Some(t.into());
        self
    }
}

pub enum NodeComponentWrapper {
    Local(Box<dyn LocalComponentWrapper>),
    Native(Box<dyn NativeComponentWrapper>),
    None,
}

pub enum NodeChildren {
    Empty,
    Text(Cow<'static, str>),
    Nodes(Vec<Node>),
}