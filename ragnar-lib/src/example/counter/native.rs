use std::borrow::Cow;

use crate::callback::TypedInputCallbackRef;
use crate::native_component::{NativeComponent, NativeEvent, NativeContext};
use crate::node::Node;
use crate::node::native_node::NativeNode;

pub struct Label {
    pub text: Cow<'static, str>,
}

pub struct Div {
    pub children: Vec<Node>,
}

#[derive(Clone)]
pub struct ClickEvent;

impl NativeEvent for ClickEvent {
    fn get_type() -> &'static str where Self: Sized {
        "example.counter.clickevent"
    }
}

pub struct Button {
    pub title: Cow<'static, str>,
    pub on_click: TypedInputCallbackRef<ClickEvent>,
}

impl NativeComponent for Label {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let _cow = self.text.clone();
        NativeNode::new("label", ctx).with_text(self.text.clone())
    }
}

impl NativeComponent for Div {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let children = self.children;

        NativeNode::new("div", ctx).with_children(children)
    }
}

impl NativeComponent for Button {
    fn render(self, mut ctx: NativeContext) -> NativeNode {
        ctx.create_chain("onClick", self.on_click);
        NativeNode::new("button", ctx).with_text(self.title)
    }
}