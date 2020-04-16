use std::borrow::Cow;
use std::include;

use ragnar_lib::{NativeCallback, NativeContext, NativeComponent, NativeEvent, NativeNode, TypedInputCallbackRef, Node};

use crate::event::MouseEvent;
use crate::form::FormId;
use crate::global::{ReferenceId, GlobalAttributes, GlobalCallbacks, NativeApply};
use crate::css::{CssStyle, CssClass};
use crate::li::Li;

#[derive(Component)]
pub struct Ol {
    pub children: Vec<Node>,

    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for Ol {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("ul",ctx)
            .with_children(self.children);

        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
