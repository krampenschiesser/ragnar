use std::borrow::Cow;
use std::collections::HashMap;

use crate::attribute::Attribute;
use crate::callback::NativeCallbackWrapper;
use crate::{INCREMENTER, TypedInputCallbackRef, NativeEvent, NativeCallback, NativeContext};
use crate::node::{Node, NodeId, TextNode};

pub struct NativeNode {
    pub id: NodeId,
    pub native_name: Cow<'static, str>,
    pub children: Vec<Node>,
    pub callbacks: Vec<NativeCallbackWrapper>,
    pub attributes: HashMap<Cow<'static, str>, Attribute>,
}

impl NativeNode {
    pub fn new<T: Into<Cow<'static, str>>>(name: T,ctx: NativeContext) -> Self {
        let id = NodeId(INCREMENTER.get_next());
        let callbacks = ctx.into_callbacks().into_iter().map(|mut c|{
            c.node_id = id;
            c
        }).collect();
        NativeNode {
            id,
            native_name: name.into(),
            children: Vec::with_capacity(0),
            callbacks,
            attributes: HashMap::new(),
        }
    }

    pub fn set_if<A: Into<Attribute>>(mut self, name: &'static str, attribute: Option<A>) -> Self {
        if let Some(attribute) = attribute {
            self.attributes.insert(name.into(), attribute.into());
        }
        self
    }

    pub fn set<T: Into<Cow<'static, str>>, A: Into<Attribute>>(mut self, name: T, attribute: A) -> Self {
        self.attributes.insert(name.into(), attribute.into());
        self
    }

    pub fn with_callback_if<T: NativeEvent, S: Into<Cow<'static, str>>>(self, name: S, forward: Option<TypedInputCallbackRef<T>>) -> NativeNode {
        if let Some(_forward) = forward {
            let callback = NativeCallback::new(name, Box::new(|e: T| e));
            self.with_callback(callback)
        } else {
            self
        }
    }
    pub fn with_callback<T: Into<NativeCallbackWrapper>>(mut self, t: T) -> Self {
        let mut callback_wrapper = t.into();
        callback_wrapper.node_id = self.id;
        self.callbacks.push(callback_wrapper);
        self
    }
    pub fn with_text<T: Into<Cow<'static, str>>>(mut self, t: T) -> Self {
        self.children.push(Node::Text(TextNode::new(t)));
        self
    }
    pub fn with_text_if<T: Into<Cow<'static, str>>>(self, t: Option<T>) -> Self {
        if let Some(t) = t {
            self.with_text(t)
        } else {
            self
        }
    }
    pub fn with_children<T: Into<Node>>(mut self, nodes: Vec<T>) -> Self {
        for node in nodes {
            self = self.with_child(node);
        }
        self
    }

    pub fn with_child<T: Into<Node>>(mut self, node: T) -> Self {
        self.children.push(node.into());
        self
    }
}

impl Into<Node> for NativeNode {
    fn into(self) -> Node {
        Node::Native(self)
    }
}