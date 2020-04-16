use crate::app_component::{AppComponent, AppEvent, AppState, AppContext};
use crate::callback::TypedInputCallbackRef;

use crate::node::app_node::AppNode;

pub struct NestedModuleLocalState {}

impl AppState for NestedModuleLocalState {}


pub enum NestedModuleLocalMsg {
    Update,
}

impl AppEvent for NestedModuleLocalMsg {}

pub struct MyNestedModuleComponent {
    pub callback: TypedInputCallbackRef<String>,
}

impl AppComponent for MyNestedModuleComponent {
    type Msg = NestedModuleLocalMsg;
    type State = NestedModuleLocalState;

    fn render(&self, _state: &Self::State,  ctx: AppContext<Self::Msg>) -> AppNode<Self::Msg> {
        AppNode::empty(ctx).into()
    }
}