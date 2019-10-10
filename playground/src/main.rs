use crate::lib::{Component, DomElement};

mod lib;

#[derive(Default, Debug)]
struct ModuleState {
    child1: ChildState1,
    child2: ChildState2,
}

#[derive(Default, Debug)]
struct ChildState1 {
    value: String,
}

#[derive(Default, Debug)]
struct ChildState2 {
    value: u32,
}

#[derive(Default)]
struct Component1 {}

impl Component for Component1 {
    type GlobalState = ModuleState;
    type LocalState = ChildState1;

    fn filter_state(state: &Self::GlobalState) -> &Self::LocalState {
        &state.child1
    }

    fn render(&self) -> DomElement {
        DomElement::default()
    }
}

#[derive(Default)]
struct Component2 {}

impl Component for Component2 {
    type GlobalState = ModuleState;
    type LocalState = ChildState2;

    fn filter_state(state: &Self::GlobalState) -> &Self::LocalState {
        &state.child2
    }

    fn render(&self) -> DomElement {
        DomElement::default()
    }
}

fn main() {
    let state = ModuleState::default();
    let component1 = Component1::default();
    let component2 = Component2::default();

    Component1::filter_state(&state);
    Component2::filter_state(&state);

    serde_json::to_string(&state);

    println!("Hello, world!");
}
