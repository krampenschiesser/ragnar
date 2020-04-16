use std::borrow::Cow;
use std::include;


use ragnar_lib::{NativeCallback, NativeComponent, NativeEvent,NativeContext, NativeNode, TypedInputCallbackRef, Node};

use crate::event::MouseEvent;
use crate::form::FormId;
use crate::global::{ReferenceId, GlobalAttributes, GlobalCallbacks, NativeApply};
use crate::input::{DataListId, CommonInputAttributes};
use chrono::{NaiveDate, Date, Utc, Datelike, NaiveTime, Timelike};

#[derive(Component, Default)]
pub struct InputTime {
    pub list: Option<DataListId>,
    pub readonly: Option<bool>,
    pub step: Option<u32>,
    pub min: Option<NaiveTime>,
    pub max: Option<NaiveTime>,
    pub value: Option<NaiveTime>,
    #[delegated]
    pub common_input_attributes: CommonInputAttributes,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for InputTime {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("input",ctx)
            .set("type", "time")
            .set_if("list", self.list)
            .set_if("readonly", self.readonly)
            .set_if("step", self.step)
            .set_if("min", self.min.map(|d| format!("{:02}{:02}", d.hour(), d.minute())))
            .set_if("max", self.max.map(|d| format!("{:02}{:02}", d.hour(), d.minute())))
            .set_if("value", self.value.map(|d| format!("{:02}{:02}", d.hour(), d.minute())))
            ;

        let node = self.common_input_attributes.apply(node);
        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
