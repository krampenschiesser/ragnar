use std::borrow::Cow;
use ragnar_lib::{NativeComponent, NativeContext, NativeNode};
use crate::global::{GlobalAttributes, GlobalCallbacks, NativeApply};
use crate::input::{CommonInputAttributes};

#[derive(Component, Default)]
pub struct InputPassword {
    pub maxlength: Option<u32>,
    pub minlength: Option<u32>,
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

impl NativeComponent for InputPassword {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("input",ctx)
            .set("type","password")
            .set_if("maxlength",self.maxlength)
            .set_if("minlength",self.minlength)
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
