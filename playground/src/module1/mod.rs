use crate::lib::{Message, DomElement, Component};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Msg {
    Switch
}

impl Message for Msg {}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct State {
    switch: bool
}

type Module = crate::lib::Module<Msg, State>;

impl crate::lib::State<Msg> for State {
    fn update(&mut self, message: Msg) {
        match message {
            Msg::Switch => self.switch = !self.switch,
        }
    }
}

struct MyComponent {}

impl Component<Msg> for MyComponent {
    type GlobalState = State;
    type LocalState = State;

    fn filter_state(state: &Self::GlobalState) -> &Self::LocalState {
        state
    }

    fn render(&self, state: &Self::LocalState) -> DomElement {
        if state.switch {
            DomElement::default()
        } else {
            DomElement::default()
        }
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