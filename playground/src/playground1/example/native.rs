use std::borrow::Cow;

use crate::playground1::callback::TypedInputCallbackRef;
use crate::playground1::native_component::NativeComponent;
use crate::playground1::node::{Node, NodeChildren, NodeComponentWrapper};

pub struct Label {
    pub text: Cow<'static, str>,
}

pub struct Div {
    pub children: Vec<Node>,
}

pub struct ClickEvent;

pub struct Button {
    pub title: Cow<'static, str>,
    pub on_click: TypedInputCallbackRef<ClickEvent>,
}

impl NativeComponent for Label {
    fn render(self) -> Node {
        let cow = self.text.clone();
        Node {
            native_name: Some("label".into()),
            component: NodeComponentWrapper::Native(Box::new(self)),
            children: NodeChildren::Text(cow),
            callbacks: Vec::with_capacity(0),
        }
    }
}

impl NativeComponent for Div {
    fn render(self) -> Node {
        let children = self.children;
        Node {
            native_name: Some("div".into()),
            component: NodeComponentWrapper::None,
            children: NodeChildren::Nodes(children),
            callbacks: Vec::with_capacity(0),
        }
    }
}

impl NativeComponent for Button {
    fn render(self) -> Node {
        Node {
            native_name: Some("div".into()),
            component: NodeComponentWrapper::None,
            children: NodeChildren::Text(self.title),
            callbacks: Vec::with_capacity(0),
        }
    }
}