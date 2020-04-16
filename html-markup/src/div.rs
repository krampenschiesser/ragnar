use std::borrow::Cow;
use std::include;


use ragnar_lib::{NativeCallback, NativeComponent, NativeEvent,NativeContext, NativeNode, TypedInputCallbackRef, Node};

use crate::event::MouseEvent;
use crate::form::FormId;
use crate::global::{ReferenceId, GlobalCallbacks, GlobalAttributes, NativeApply};
use crate::css::{CssStyle, CssClass};

#[derive(Component, Default)]
pub struct Div {
    pub children: Vec<Node>,

    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for Div {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("div",ctx)
            .with_children(self.children);

        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
