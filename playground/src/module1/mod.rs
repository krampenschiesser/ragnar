use serde::{Deserialize, Serialize};

use crate::{Component, DomElement, GlobalRender, Message, Renderable};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Msg {
    Switch
}

impl Message for Msg {}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct State {
    switch: bool
}

type Module = crate::Module<Msg, State>;

impl crate::State<Msg> for State {
    fn update(&mut self, message: Msg) {
        match message {
            Msg::Switch => self.switch = !self.switch,
        }
    }
}

struct MyComponent {}

impl Component<Msg, State> for MyComponent {
    fn render(&self, state: &State) -> Renderable {
        unimplemented!()
        // let element: DomElement<Msg> = DomElement::default();
        // if state.switch {
        //     element
        // } else {
        //     element
        // }.into()
    }
}

#[no_mangle]
pub extern fn init(tuple: Option<(&str, &str, &str)>) -> Module {
    if let Some(tuple) = tuple {
        let (initial, current, messages) = tuple;
        let result = Module::from_json(initial, current, messages);
        if let Ok(module) = result {
            return module;
        }
    }
    Module::new()
}