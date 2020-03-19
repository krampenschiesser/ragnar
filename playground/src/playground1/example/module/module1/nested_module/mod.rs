use crate::playground1::app_component::{AppComponent, AppEvent, AppState};
use crate::playground1::callback::TypedInputCallbackRef;
use crate::playground1::node::Node;

pub struct NestedModuleLocalState {}

impl AppState for NestedModuleLocalState {}


pub enum NestedModuleLocalMsg {
    Update,
}

impl AppEvent for NestedModuleLocalMsg {}

pub struct MyNestedModuleComponent {
    callback: TypedInputCallbackRef<String>,
}

impl AppComponent for MyNestedModuleComponent {
    type Msg = NestedModuleLocalMsg;
    type State = NestedModuleLocalState;

    fn render(&self, state: &Self::State) -> Node {
        Node::empty()
    }
}