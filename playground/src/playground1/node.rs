use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;

use crate::playground1::app_component::AppEvent;
use crate::playground1::attribute::Attribute;
use crate::playground1::callback::CallbackWrapper;
use crate::playground1::INCREMENTER;
use crate::playground1::local_component::LocalComponentWrapper;
use crate::playground1::native_component::{NativeComponent, NativeComponentWrapper};
use crate::playground1::node::NodeComponentWrapper::LocalDetached;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct NodeId(pub u64);

impl Deref for NodeId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait BaseNode: Sized {
    fn empty() -> Self;

    fn with_callback<T: Into<CallbackWrapper>>(self, t: T) -> Self;
    fn with_children<T: Into<Self>>(self, nodes: Vec<T>) -> Self;
    fn with_child<T: Into<Self>>(self, node: T) -> Self;
}

pub struct TypedNode<T: AppEvent + 'static> {
    node: Node,
    _phantom: PhantomData<T>,
}

impl<U: AppEvent + 'static> TypedNode<U> {
    pub(crate) fn into_node(self) -> Node {
        self.node
    }
    pub fn empty() -> Self {
        Self {
            node: Node::empty(),
            _phantom: PhantomData,
        }
    }


    pub fn with_callback<T: Into<CallbackWrapper>>(self, t: T) -> Self {
        let Self { node, _phantom } = self;
        let node = node.with_callback(t);
        Self {
            node,
            _phantom,
        }
    }

    pub fn with_children<T: Into<TypedNode<U>>>(self, nodes: Vec<T>) -> Self {
        let Self { node, _phantom } = self;
        let vec: Vec<_> = nodes.into_iter().map(|s| s.into().into_node()).collect();
        let node = node.with_children(vec);
        Self {
            node,
            _phantom,
        }
    }

    pub fn with_child_and_converter<O: AppEvent + 'static, T: Into<TypedNode<O>>>(self, node: T, converter: impl Fn(O) -> U + 'static) -> Self {
        let Self { node: s, _phantom } = self;
        let child_node = node.into().into_node();
        let converter_wrapped = Box::new(move |a: Box<dyn AppEvent>| {
            if let Some(e) = a.downcast::<O>().ok() {
                Some(converter(*e));
            };
            None
        });
        let child_node = child_node.with_converter(converter_wrapped);
        let node = s.with_child(child_node);
        Self {
            node,
            _phantom,
        }
    }
    pub fn with_child<T: Into<TypedNode<U>>>(self, node: T) -> Self {
        let Self { node: s, _phantom } = self;
        let node = s.with_child(node.into().into_node());
        Self {
            node,
            _phantom,
        }
    }
}

impl<T: AppEvent> From<Node> for TypedNode<T> {
    fn from(n: Node) -> Self {
        Self {
            node: n,
            _phantom: PhantomData,
        }
    }
}

pub type Converter = Box<dyn Fn(Box<dyn AppEvent>) -> Option<Box<dyn AppEvent>>>;

pub struct Node {
    pub id: NodeId,
    pub native_name: Option<Cow<'static, str>>,
    pub component: NodeComponentWrapper,
    pub children: NodeChildren,
    pub callbacks: Vec<CallbackWrapper>,
    pub attributes: HashMap<Cow<'static, str>, Attribute>,
    pub converter: Option<Converter>,
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
            converter: None,
        }
    }

    pub fn set<T: Into<Cow<'static, str>>, A: Into<Attribute>>(mut self, name: T, attribute: A) -> Self {
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
    pub fn with_local_component<T: LocalComponentWrapper + 'static>(mut self, t: T) -> Self {
        self.component = NodeComponentWrapper::Local(Box::new(t));
        self
    }
    pub fn with_native_component<T: NativeComponent + 'static>(mut self, t: T) -> Self {
        self.component = NodeComponentWrapper::Native(Box::new(t));
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
    pub fn with_native_name<T: Into<Cow<'static, str>>>(mut self, t: T) -> Self {
        self.native_name = Some(t.into());
        self
    }
    pub(crate) fn with_converter(mut self, converter: Box<dyn Fn(Box<dyn AppEvent>) -> Option<Box<dyn AppEvent>>>) -> Self {
        self.converter = Some(converter);
        self
    }
}

pub enum NodeComponentWrapper {
    Local(Box<dyn LocalComponentWrapper>),
    LocalDetached,
    Native(Box<dyn NativeComponentWrapper>),
    None,
}

pub enum NodeChildren {
    Empty,
    Text(Cow<'static, str>),
    Nodes(Vec<Node>),
}