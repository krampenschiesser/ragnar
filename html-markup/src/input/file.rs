use std::borrow::Cow;
use ragnar_lib::{NativeComponent, NativeContext, NativeNode};
use crate::global::{GlobalAttributes, GlobalCallbacks, NativeApply};
use crate::input::CommonInputAttributes;

#[derive(Component, Default)]
pub struct InputFile {
    pub required: Option<bool>,
    pub accept: Option<Cow<'static, str>>,
    pub capture: Option<Cow<'static, str>>,
    pub multiple: Option<bool>,
    #[delegated]
    pub common_input_attributes: CommonInputAttributes,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for InputFile {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("input",ctx)
            .set("type","file")
            .set_if("required",self.required)
            .set_if("accept",self.accept)
            .set_if("capture",self.capture)
            .set_if("multiple",self.multiple);

        let node = self.common_input_attributes.apply(node);
        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
