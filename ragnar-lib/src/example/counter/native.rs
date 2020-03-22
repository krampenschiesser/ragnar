use std::borrow::Cow;

use crate::callback::TypedInputCallbackRef;
use crate::native_component::{NativeComponent, NativeEvent};
use crate::node::{Node};
use crate::node::native_node::NativeNode;

pub struct Label {
    pub text: Cow<'static, str>,
}

pub struct Div {
    pub children: Vec<Node>,
}

#[derive(Clone)]
pub struct ClickEvent;

impl NativeEvent for ClickEvent {}

pub struct Button {
    pub title: Cow<'static, str>,
    pub on_click: TypedInputCallbackRef<ClickEvent>,
}

impl NativeComponent for Label {
    fn render(self) -> NativeNode {
        let _cow = self.text.clone();
        NativeNode::new("label").with_text(self.text.clone())
    }
}

impl NativeComponent for Div {
    fn render(self) -> NativeNode {
        let children = self.children;

        NativeNode::new("div").with_children(children)
    }
}

impl NativeComponent for Button {
    fn render(self) -> NativeNode {
        let mut callback = Self::create_native_callback("onClick", Box::new(|event: ClickEvent| event));
        callback.chain(self.on_click);
        NativeNode::new("button").with_text(self.title).with_callback(callback)
    }
}