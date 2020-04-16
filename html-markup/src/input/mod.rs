use std::borrow::Cow;
use ragnar_lib::{Attribute, NativeNode};
use crate::global::NativeApply;
use crate::form::FormId;
use crate::input::auto_complete::AutoComplete;

pub mod file;
pub mod button;
pub mod check_box;
pub mod color;
pub mod date;
pub mod datetime;
pub mod email;
pub mod hidden;
pub mod image;
pub mod month;
pub mod number;
pub mod password;
pub mod radio;
pub mod range;
pub mod reset;
pub mod search;
pub mod submit;
pub mod tel;
pub mod text;
pub mod time;
pub mod url;

pub mod auto_complete;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DataListId(Cow<'static, str>);


impl Into<Attribute> for DataListId {
    fn into(self) -> Attribute {
        Attribute::Text(self.0.into())
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct FormInputAttributes {
    pub formaction: Option<Cow<'static, str>>,
    pub formenctype: Option<Cow<'static, str>>,
    pub formmethod: Option<Cow<'static, str>>,
    pub formnovalidate: Option<bool>,
    pub formtarget: Option<Cow<'static, str>>,
}

#[derive(Component, Default, Debug, Clone)]
pub struct CommonInputAttributes {
    pub autocomplete: Option<AutoComplete>,
    pub autofocus: Option<bool>,
    pub disabled: Option<bool>,
    pub form: Option<FormId>,
    pub name: Option<Cow<'static, str>>,
}

impl NativeApply for FormInputAttributes {
    fn apply(self, node: NativeNode) -> NativeNode {
        let node = node.set_if("formaction", self.formaction)
            .set_if("formenctype", self.formenctype)
            .set_if("formmethod", self.formmethod)
            .set_if("formnovalidate", self.formnovalidate)
            .set_if("formtarget", self.formtarget);
        node
    }
}

impl NativeApply for CommonInputAttributes {
    fn apply(self, node: NativeNode) -> NativeNode {
        let node = node.set_if("autocomplete", self.autocomplete.map(|a|a.to_static_str()))
            .set_if("autofocus", self.autofocus)
            .set_if("disabled", self.disabled)
            .set_if("form", self.form)
            .set_if("name", self.name);
        node
    }
}