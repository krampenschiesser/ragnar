use std::ops::Deref;

use ragnar_lib::{Attribute, TypedInputCallbackRef, NativeNode};

use crate::css::{CssClass, CssStyle};
use crate::event::{MouseEvent, InputEvent};
use std::borrow::Cow;
use std::collections::HashMap;
use crate::event::keyboard::KeyboardEvent;

pub mod file;

#[derive(Debug)]
pub struct ReferenceId(String);

impl Deref for ReferenceId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<String> for ReferenceId {
    fn into(self) -> String {
        self.0
    }
}


impl Into<Attribute> for ReferenceId {
    fn into(self) -> Attribute {
        Attribute::Text(self.0.into())
    }
}

impl From<&str> for ReferenceId {
    fn from(val: &str) -> Self {
        ReferenceId(val.into())
    }
}

#[derive(Debug, Default, Component)]
pub struct GlobalAttributes {
    pub access_key: Option<char>,
    pub auto_capitalize: Option<Capitalization>,
    pub class: Option<CssClass>,
    pub style: Option<CssStyle>,
    pub id: Option<ReferenceId>,
    pub additional_attributes: Option<HashMap<Cow<'static, str>, Attribute>>,
}

#[derive(Debug, Default, Component)]
pub struct GlobalCallbacks {
    #[rename("onkeydown")]
    pub on_key_down: Option<TypedInputCallbackRef<KeyboardEvent>>,
    #[rename("onkeypress")]
    pub on_key_press: Option<TypedInputCallbackRef<KeyboardEvent>>,
    #[rename("onkeyup")]
    pub on_key_up: Option<TypedInputCallbackRef<KeyboardEvent>>,
    #[rename("oninput")]
    pub on_input: Option<TypedInputCallbackRef<InputEvent>>,
    #[rename("onclick")]
    pub on_click: Option<TypedInputCallbackRef<MouseEvent>>,
    #[rename("onmousedown")]
    pub on_mouse_down: Option<TypedInputCallbackRef<MouseEvent>>,
    #[rename("onmouseenter")]
    pub on_mouse_enter: Option<TypedInputCallbackRef<MouseEvent>>,
    #[rename("onmouseleave")]
    pub on_mouse_leave: Option<TypedInputCallbackRef<MouseEvent>>,
    #[rename("onmousemove")]
    pub on_mouse_move: Option<TypedInputCallbackRef<MouseEvent>>,
    #[rename("onmouseout")]
    pub on_mouse_out: Option<TypedInputCallbackRef<MouseEvent>>,
    #[rename("onmouseover")]
    pub on_mouse_over: Option<TypedInputCallbackRef<MouseEvent>>,
    #[rename("onmouseup")]
    pub on_mouse_up: Option<TypedInputCallbackRef<MouseEvent>>,
}

#[derive(Debug)]
pub enum Capitalization {
    Off,
    None,
    On,
    Sentences,
    Words,
    Characters,
}

impl Into<Attribute> for Capitalization {
    fn into(self) -> Attribute {
        let str = match self {
            Capitalization::Off => "off",
            Capitalization::None => "none",
            Capitalization::On => "on",
            Capitalization::Sentences => "sentences",
            Capitalization::Words => "words",
            Capitalization::Characters => "characters"
        };
        Attribute::Text(str.into())
    }
}

pub trait NativeApply {
    fn apply(self, node: NativeNode) -> NativeNode;
}

impl NativeApply for GlobalAttributes {
    fn apply(self, node: NativeNode) -> NativeNode {
        let GlobalAttributes {
            access_key,
            auto_capitalize,
            class,
            style,
            id,
            additional_attributes
        } = self;
        let mut node = node.set_if("accessKey", access_key)
            .set_if("class", class)
            .set_if("autoCapitalize", auto_capitalize)
            .set_if("style", style)
            .set_if("id", id);


        if let Some(additional_attributes) = additional_attributes {
            for (key, value) in additional_attributes {
                node = node.set(key, value);
            }
        }
        node
    }
}

impl NativeApply for GlobalCallbacks {
    fn apply(self, node: NativeNode) -> NativeNode {
        let GlobalCallbacks {
            on_key_down,
            on_key_press,
            on_key_up,
            on_input,
            on_click,
            on_mouse_down,
            on_mouse_enter,
            on_mouse_leave,
            on_mouse_move,
            on_mouse_out,
            on_mouse_over,
            on_mouse_up,
        } = self;
        node.with_callback_if("onclick", on_click)
            .with_callback_if("onkeydown", on_key_down)
            .with_callback_if("onkeypress", on_key_press)
            .with_callback_if("onkeyup", on_key_up)
            .with_callback_if("oninput", on_input)
            .with_callback_if("onmousedown", on_mouse_down)
            .with_callback_if("onmouseenter", on_mouse_enter)
            .with_callback_if("onmouseleave", on_mouse_leave)
            .with_callback_if("onmousemove", on_mouse_move)
            .with_callback_if("onmouseout", on_mouse_out)
            .with_callback_if("onmouseover", on_mouse_over)
            .with_callback_if("onmouseup", on_mouse_up)
    }
}