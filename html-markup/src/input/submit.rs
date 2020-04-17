use std::borrow::Cow;
use ragnar_lib::{NativeComponent, NativeContext, NativeNode};
use crate::global::{GlobalAttributes, GlobalCallbacks, NativeApply};
use crate::input::{FormInputAttributes, CommonInputAttributes};

#[derive(Component, Default)]
pub struct InputSubmit {
    pub value: Option<Cow<'static, str>>,
    #[delegated]
    pub common_input_attributes: CommonInputAttributes,
    #[delegated]
    pub form_attributes: FormInputAttributes,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for InputSubmit {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("input",ctx)
            .set("type","submit")
            .set_if("value",self.value);

        let node = self.common_input_attributes.apply(node);
        let node = self.form_attributes.apply(node);
        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
