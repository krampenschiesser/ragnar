use ragnar_lib::{NativeComponent, NativeContext, NativeNode};
use crate::global::{GlobalAttributes, GlobalCallbacks, NativeApply};
use crate::input::{DataListId, CommonInputAttributes};

#[derive(Component, Default)]
pub struct InputColor {
    pub list: Option<DataListId>,
    #[delegated]
    pub common_input_attributes: CommonInputAttributes,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for InputColor {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("input",ctx)
            .set("type","color")
            .set_if("list",self.list);

        let node = self.common_input_attributes.apply(node);
        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
