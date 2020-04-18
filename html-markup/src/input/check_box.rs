use std::borrow::Cow;
use ragnar_lib::{NativeComponent, NativeContext, NativeNode};
use crate::global::{GlobalAttributes, GlobalCallbacks, NativeApply};
use crate::input::CommonInputAttributes;

#[derive(Component, Default)]
pub struct InputCheckBox {
    pub value: Option<Cow<'static, str>>,
    pub checked: Option<bool>,
    pub readonly: Option<bool>,
    #[delegated]
    pub common_input_attributes: CommonInputAttributes,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for InputCheckBox {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("input",ctx)
            .set("type","checkbox")
            .set_if("value",self.value)
            .set_if("readonly",self.readonly)
            .set_if("checked",self.checked);

        let node = self.common_input_attributes.apply(node);
        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
