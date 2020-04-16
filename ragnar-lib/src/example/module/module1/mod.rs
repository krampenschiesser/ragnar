use crate::app_component::{AppComponent, AppEvent, AppState, AppContext};
use crate::callback::TypedInputCallbackRef;
use crate::example::module::module1::nested_module::{MyNestedModuleComponent, NestedModuleLocalMsg, NestedModuleLocalState};

use crate::node::app_node::AppNode;

mod nested_module;


pub struct ModuleLocalState {
    nested: NestedModuleLocalState,
}

impl AppState for ModuleLocalState {}


pub enum ModuleLocalMsg {
    Nested(NestedModuleLocalMsg),
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

    fn render(&self, state: &Self::State, ctx: AppContext<Self::Msg>) -> AppNode<Self::Msg> {
        let nested = MyNestedModuleComponent { callback: self.callback.clone() };
        let f = |e: NestedModuleLocalMsg| {
            ModuleLocalMsg::Nested(e)
        };
        let node = nested.render(&state.nested,AppContext::new());
        AppNode::empty(ctx).with_child_and_converter(node, f)
    }
}