use serde::export::fmt::{Debug, Error};
use serde::export::Formatter;
use std::collections::HashMap;

use crate::{Component, GlobalRender, Message, Renderable, State};

mod node;
mod component;
mod component2;
mod component3;
mod component4;
mod component5;

struct Div<MSG: Message, STATE: State<MSG>> {
    children: Vec<Box<dyn Component<MSG, STATE>>>
}

struct Button<MSG: Message> {
    title: String,
    on_click: Box<dyn Fn() -> MSG>,
}

struct TextField<MSG: Message> {
    text: String,
    on_text_change: Option<Box<dyn Fn(&str) -> MSG>>,
}

impl<MSG: Message, STATE: State<MSG>> Component<MSG, STATE> for TextField<MSG> {
    fn render(&self, state: &STATE) -> Renderable {
        unimplemented!()
    }
}

#[derive(Default, Debug)]
pub struct DomElement<MSG: Message> {
    name: String,
    children: Vec<Box<DomElement<MSG>>>,
    props: HashMap<String, DomProperty<MSG>>,
}


impl<MSG: Message> GlobalRender for DomElement<MSG> {}

impl<MSG: Message> From<DomElement<MSG>> for Renderable {
    fn from(_: DomElement<MSG>) -> Self {
        Renderable{}
    }
}

//impl<MSG: Message> From<TextField<MSG>> for DomElement<MSG> {
//    fn from(input: TextField<MSG>) -> Self {
//        let mut props = HashMap::new();
//        props.insert("text".into(), DomProperty::Text(input.text));
//        if let Some(callback) = input.on_text_change {
//            MyCallback {
//                callback,
//            };
//            props.insert("on_text_change".into(), DomProperty::CallBack(callback));
//        }
//        DomElement {
//            name: "TextField".into(),
//            children: Vec::new(),
//            props,
//        };
//        unimplemented!()
//    }
//}

enum CallbackInput {
    Text(String)
}

impl From<&str> for CallbackInput {
    fn from(input: &str) -> Self {
        CallbackInput::Text(input.into())
    }
}

impl AsRef<str> for CallbackInput {
    fn as_ref(&self) -> &str {
        match self {
            CallbackInput::Text(something) => &something
        }
    }
}

struct MyCallback<OUT: Message> {
    callback: Box<dyn Fn(CallbackInput) -> OUT>,
}

pub enum DomProperty<MSG: Message> {
    CallBack(MyCallback<MSG>),
    Text(String),
    Boolean(bool),
    U32(u32),
    F32(f32),
}

impl<MSG: Message> Debug for DomProperty<MSG> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    enum Msg {}

    impl Message for Msg {}

    #[derive(Serialize, Deserialize, Clone, Debug, Default)]
    struct State {}

    impl crate::State<Msg> for State {
        fn update(&mut self, message: Msg) {
            unimplemented!()
        }
    }

    #[test]
    fn simple_dom() {
        let text_field: TextField<Msg> = TextField {
            on_text_change: None,
            text: "hello world".into(),
        };
        let children: Vec<DomElement<Msg>> = vec![
            DomElement {
                name: "button".into(),
                children: Vec::new(),
                props: HashMap::new(),
            }
        ];
        let element: DomElement<Msg> = DomElement {
            name: "div".into(),
            children: Vec::new(),
            props: HashMap::new(),
        };
    }
}