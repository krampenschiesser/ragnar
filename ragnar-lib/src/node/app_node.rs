use std::marker::PhantomData;

use crate::app_component::{AppEvent, AppContext};
use crate::callback::AppCallbackWrapper;
use crate::INCREMENTER;
use crate::node::{Node, NodeId};

pub struct AppNode<T: AppEvent + 'static> {
    internal: UntypedAppNode,
    _phantom: PhantomData<T>,
}

pub struct UntypedAppNode {
    pub id: NodeId,
    pub children: Option<Vec<Node>>,
    pub callbacks: Vec<AppCallbackWrapper>,
    pub converter: Option<Converter>,
}

impl<U: AppEvent + 'static> AppNode<U> {
    pub fn empty(ctx: AppContext<U>) -> Self {
        let id = NodeId(INCREMENTER.get_next());
        let callbacks = ctx.into_callbacks().into_iter().map(|mut c| {
            c.node_id = id;
            c
        }).collect();
        let internal = UntypedAppNode {
            id,
            children: None,
            callbacks,
            converter: None,
        };
        Self {
            internal,
            _phantom: PhantomData,
        }
    }

    pub fn with_children<T: Into<Node>>(mut self, nodes: Vec<T>) -> Self {
        for node in nodes {
            self = self.with_child(node);
        }
        self
    }

    pub fn with_child<T: Into<Node>>(mut self, node: T) -> Self {
        if self.internal.children.is_none() {
            self.internal.children = Some(Vec::with_capacity(1));
        }
        if let Some(children) = &mut self.internal.children {
            children.push(node.into());
        }
        self
    }
    pub fn with_converter(mut self, converter: Box<dyn Fn(Box<dyn AppEvent>) -> Option<Box<dyn AppEvent>>>) -> Self {
        self.internal.converter = Some(converter);
        self
    }

    pub fn with_child_and_converter<O: AppEvent + 'static, T: Into<AppNode<O>>>(self, node: T, converter: impl Fn(O) -> U + 'static) -> Self {
        let child_node = node.into();
        let converter_wrapped = Box::new(move |a: Box<dyn AppEvent>| {
            if let Some(e) = a.downcast::<O>().ok() {
                Some(converter(*e));
            };
            None
        });
        let child_node = child_node.with_converter(converter_wrapped);
        let node = self.with_child(Node::App(child_node.internal));
        node
    }
}

pub type Converter = Box<dyn Fn(Box<dyn AppEvent>) -> Option<Box<dyn AppEvent>>>;

impl<T: AppEvent + 'static> Into<Node> for AppNode<T> {
    fn into(self) -> Node {
        Node::App(self.internal)
    }
}

impl Into<Node> for UntypedAppNode {
    fn into(self) -> Node {
        Node::App(self)
    }
}

impl<T: AppEvent + 'static> Into<UntypedAppNode> for AppNode<T> {
    fn into(self) -> UntypedAppNode {
        self.internal
    }
}