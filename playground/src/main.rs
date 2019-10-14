#[macro_use]
extern crate log;

use serde::{Serialize, Deserialize};
use crate::lib::{Component, DomElement, State, Message};

mod lib;
mod module1;
mod runner;
//mod module2;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct ModuleState {
    child1: ChildState1,
    child2: ChildState2,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct ChildState1 {
    value: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct ChildState2 {
    value: u32,
}

#[derive(Default)]
struct Component1 {}

impl Component<Msg1> for Component1 {
    type GlobalState = ModuleState;
    type LocalState = ChildState1;

    fn filter_state(state: &Self::GlobalState) -> &Self::LocalState {
        &state.child1
    }

    fn render(&self, state: &ChildState1) -> DomElement {
        DomElement::default()
    }
}

#[derive(Default)]
struct Component2 {}

impl Component<Msg1> for Component2 {
    type GlobalState = ModuleState;
    type LocalState = ChildState2;

    fn filter_state(state: &Self::GlobalState) -> &Self::LocalState {
        &state.child2
    }

    fn render(&self, state: &ChildState2) -> DomElement {
        DomElement::default()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Msg1 {
    Value(String)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Msg2 {
    Value(u32)
}

impl Message for Msg1 {}

impl Message for Msg2 {}

impl State<Msg1> for ModuleState {
    fn update(&mut self, message: Msg1) {
        match message {
            Msg1::Value(x) => self.child1.value = x
        }
    }
}

impl State<Msg2> for ModuleState {
    fn update(&mut self, message: Msg2) {
        match message {
            Msg2::Value(x) => self.child2.value = x
        }
    }
}

impl State<Msg1> for ChildState1 {
    fn update(&mut self, message: Msg1) {
        unimplemented!()
    }
}

impl State<Msg2> for ChildState2 {
    fn update(&mut self, message: Msg2) {
        unimplemented!()
    }
}

fn main() {
    let mut state = ModuleState::default();
    let component1 = Component1::default();
    let component2 = Component2::default();

    component1.render(Component1::filter_state(&mut state));
    component2.render(Component2::filter_state(&mut state));

    serde_json::to_string(&state).unwrap();

    let msg1 = Msg1::Value("bla".into());
    let msg2 = Msg2::Value(42);

    state.update(msg1.clone());
    state.update(msg2.clone());

//    let vec: Vec<Box<dyn Message> > = Vec::new();
    println!("Hello, world!");
}
