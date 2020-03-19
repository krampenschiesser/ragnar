use crate::playground1::app_component::{AppComponent, AppEvent, AppState};
use crate::playground1::callback::TypedInputCallbackRef;
use crate::playground1::node::Node;

mod nested_module;


pub struct ModuleLocalState {}

impl AppState for ModuleLocalState {}


pub enum ModuleLocalMsg {

    Insert,
    Remove,
}

impl AppEvent for ModuleLocalMsg {}

pub struct MyModuleComponent {
    pub callback: TypedInputCallbackRef<String>,
}

impl AppComponent for MyModuleComponent {
    type Msg = ModuleLocalMsg;
    type State = ModuleLocalState;

    fn render(&self, state: &Self::State) -> Node {
        Node::empty()
    }
}