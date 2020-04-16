use std::borrow::Cow;
use std::include;


use ragnar_lib::{NativeCallback, NativeComponent, NativeEvent,NativeContext, NativeNode, TypedInputCallbackRef, Node};

use crate::event::MouseEvent;
use crate::form::FormId;
use crate::global::{ReferenceId, GlobalAttributes, GlobalCallbacks, NativeApply};
use crate::input::{DataListId, CommonInputAttributes};

#[derive(Component, Default)]
pub struct InputEmail {
    pub list: Option<DataListId>,
    pub maxlength: Option<u32>,
    pub minlength: Option<u32>,
    pub multiple: Option<bool>,
    pub pattern: Option<Cow<'static, str>>,
    pub placeholder: Option<Cow<'static, str>>,
    pub value: Option<Cow<'static, str>>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub size: Option<u16>,
    #[delegated]
    pub common_input_attributes: CommonInputAttributes,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for InputEmail {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("input",ctx)
            .set("type","email")
            .set_if("list",self.list)
            .set_if("maxlength",self.maxlength)
            .set_if("minlength",self.minlength)
            .set_if("multiple",self.multiple)
            .set_if("pattern",self.pattern)
            .set_if("placeholder",self.placeholder)
            .set_if("readonly",self.readonly)
            .set_if("required",self.required)
            .set_if("size",self.size)
            .set_if("value",self.value)
            ;

        let node = self.common_input_attributes.apply(node);
        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
