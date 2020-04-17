use ragnar_lib::{NativeComponent, NativeContext, NativeNode};
use crate::global::{GlobalAttributes, GlobalCallbacks, NativeApply};
use crate::input::{DataListId, CommonInputAttributes};
use chrono::{Date, Utc, Datelike};

#[derive(Component, Default)]
pub struct InputDate {
    pub list: Option<DataListId>,
    pub readonly: Option<bool>,
    pub step: Option<u16>,
    pub min: Option<Date<Utc>>,
    pub max: Option<Date<Utc>>,
    pub value: Option<Date<Utc>>,
    #[delegated]
    pub common_input_attributes: CommonInputAttributes,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for InputDate {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("input",ctx)
            .set("type", "month")
            .set_if("list", self.list)
            .set_if("readonly", self.readonly)
            .set_if("step", self.step)
            .set_if("min", self.min.map(|d| format!("{}{:02}", d.year(), d.month())))
            .set_if("max", self.max.map(|d| format!("{}{:02}", d.year(), d.month())))
            .set_if("value", self.value.map(|d| format!("{}{:02}", d.year(), d.month())))
            ;

        let node = self.common_input_attributes.apply(node);
        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
