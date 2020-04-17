use ragnar_lib::{NativeComponent, NativeContext, NativeNode};
use crate::global::{GlobalAttributes, GlobalCallbacks, NativeApply};
use crate::input::{DataListId, CommonInputAttributes};

#[derive(Component, Default)]
pub struct InputRange {
    pub list: Option<DataListId>,
    pub value: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    #[delegated]
    pub common_input_attributes: CommonInputAttributes,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for InputRange {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("input",ctx)
            .set("type","range")
            .set_if("list",self.list)
            .set_if("value",self.value)
            .set_if("min",self.min)
            .set_if("max",self.max)
            .set_if("step",self.step)
            ;

        let node = self.common_input_attributes.apply(node);
        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
