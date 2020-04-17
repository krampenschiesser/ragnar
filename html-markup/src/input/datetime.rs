use ragnar_lib::{NativeComponent, NativeContext, NativeNode};
use crate::global::{GlobalAttributes, GlobalCallbacks, NativeApply};
use crate::input::{DataListId, CommonInputAttributes};
use chrono::{Datelike, DateTime, Local, Timelike};

#[derive(Component, Default)]
pub struct InputDateTimeLocal {
    pub list: Option<DataListId>,
    pub readonly: Option<bool>,
    pub step: Option<u32>,
    pub min: Option<DateTime<Local>>,
    pub max: Option<DateTime<Local>>,
    pub value: Option<DateTime<Local>>,
    #[delegated]
    pub common_input_attributes: CommonInputAttributes,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for InputDateTimeLocal {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("input",ctx)
            .set("type", "datetime-local")
            .set_if("list", self.list)
            .set_if("readonly", self.readonly)
            .set_if("step", self.step)
            .set_if("min", self.min.map(|d| format!("{}{:02}{:02}T{:02}:{:02}", d.year(), d.month(), d.day(), d.hour(), d.minute())))
            .set_if("max", self.max.map(|d| format!("{}{:02}{:02}T{:02}:{:02}", d.year(), d.month(), d.day(), d.hour(), d.minute())))
            .set_if("value", self.value.map(|d| format!("{}{:02}{:02}T{:02}:{:02}", d.year(), d.month(), d.day(), d.hour(), d.minute())))
            ;

        let node = self.common_input_attributes.apply(node);
        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
