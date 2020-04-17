use ragnar_lib::{NativeComponent, NativeContext, NativeNode};
use crate::global::{GlobalAttributes, GlobalCallbacks, NativeApply};
use crate::input::{DataListId, CommonInputAttributes};

#[derive(Component, Default)]
pub struct InputNumber {
    pub list: Option<DataListId>,
    pub placeholder: Option<f64>,
    pub value: Option<f64>,
    pub step: Option<f64>,
    pub readonly: Option<bool>,
    #[delegated]
    pub common_input_attributes: CommonInputAttributes,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for InputNumber {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("input", ctx)
            .set("type", "number")
            .set_if("list", self.list)
            .set_if("placeholder", self.placeholder)
            .set_if("value", self.value)
            .set_if("readonly", self.readonly)
            .set_if("step", self.step)
            ;

        let node = self.common_input_attributes.apply(node);
        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
