use downcast_rs::{Downcast, impl_downcast};
use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;

use crate::dom::component4::AppState;

pub struct Node;

pub type Attribute = String;

pub struct NativeComponent {
    id: u64,
    name: Cow<'static, str>,
    attributes: HashMap<Cow<'static, str>, Attribute>,
    callbacks: Vec<NativeCallback>,
}

struct OnClick {}

impl NativeEvent for OnClick {}

pub fn btn(name: &'static str, callback: Box<dyn Fn(&OnClick) -> CallbackResult>) -> NativeComponent {
    let wrapper = move |event: Rc<dyn NativeEvent>| {
        if let Some(event) = event.downcast_ref::<OnClick>() {
            callback(event)
        } else {
            CallbackResult::Error("could not convert event".into())
        }
    };
    let callback = NativeCallback {
        id: 0,
        mount: "onClick".into(),
        handler: Box::new(wrapper),
    };
    NativeComponent {
        id: 0,
        name: name.into(),
        attributes: HashMap::new(),
        callbacks: vec![callback],
    }
}

pub enum CallbackResult {
    AppEvent(Box<dyn AppEvent>),
    LocalEvent(Box<dyn LocalEvent>),
    Error(Cow<'static, str>),
}

pub trait NativeEvent: Downcast {}
impl_downcast!(NativeEvent);

pub trait AppEvent {}

pub trait LocalEvent {}

pub struct NativeCallback {
    id: u64,
    mount: Cow<'static, str>,
    handler: Box<dyn Fn(Rc<dyn NativeEvent>) -> CallbackResult>,
}

pub trait LocalComponent {
    type Msg: LocalEvent;

    fn render(&self) -> Node;

    fn update(&mut self, msg: &Self::Msg) -> UpdateResult;

    // fn handle()
}

pub trait AppComponent {
    type Msg: AppEvent;
    type State;

    fn render(&self, state: &AppState) -> Node;
}

pub enum UpdateResult {
    Render,
    Ignore,
}


struct Button<T> {
    call_back: Box<dyn Fn(ButtonMsg) -> T>
}

struct ButtonMsg;

impl LocalEvent for ButtonMsg {}

impl<T> LocalComponent for Button<T> {
    type Msg = ButtonMsg;

    fn render(&self) -> Node {
        unimplemented!()
    }

    fn update(&mut self, msg: &Self::Msg) -> UpdateResult {
        unimplemented!()
    }
}