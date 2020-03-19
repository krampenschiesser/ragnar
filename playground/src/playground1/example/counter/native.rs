use std::borrow::Cow;

use crate::playground1::callback::TypedInputCallbackRef;
use crate::playground1::native_component::{NativeComponent, NativeEvent};
use crate::playground1::node::{Node};

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
    fn render(self) -> Node {
        let _cow = self.text.clone();
        Node::empty().with_text(self.text.clone()).with_native_name("label").with_native_component(self)
    }
}

impl NativeComponent for Div {
    fn render(self) -> Node {
        let children = self.children;
        Node::empty().with_children(children).with_native_name("div")
    }
}

impl NativeComponent for Button {
    fn render(self) -> Node {
        let mut callback = Self::create_native_callback("onClick", Box::new(|event: &ClickEvent| event.clone()));
        callback.chain(self.on_click);
        Node::empty().with_native_name("button").with_text(self.title).with_callback(callback)
    }
}