use std::borrow::Cow;
use std::include;


use ragnar_lib::{NativeCallback, NativeComponent, NativeEvent,NativeContext, NativeNode, TypedInputCallbackRef, Node};

use crate::event::MouseEvent;
use crate::form::FormId;
use crate::global::{ReferenceId, GlobalAttributes, GlobalCallbacks, NativeApply};
use crate::input::CommonInputAttributes;

#[derive(Component, Default)]
pub struct InputRadio {
    pub value: Option<Cow<'static, str>>,
    pub checked: Option<bool>,
    #[delegated]
    pub common_input_attributes: CommonInputAttributes,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for InputRadio {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("input",ctx)
            .set("type", "radio")
            .set_if("value", self.value)
            .set_if("checked", self.checked);

        let node = self.common_input_attributes.apply(node);
        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
