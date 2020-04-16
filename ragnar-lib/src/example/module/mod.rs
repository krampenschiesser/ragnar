

use crate::app_component::{AppComponent, AppEvent, AppState, AppContext};
use crate::callback::TypedInputCallbackRef;
use crate::example::module::module1::{ModuleLocalMsg, ModuleLocalState, MyModuleComponent};

use crate::node::app_node::AppNode;

mod module1;

pub struct GlobalState {
    module_state: ModuleLocalState
}

impl AppState for GlobalState {}


pub enum GlobalMsg {
    Local(ModuleLocalMsg),
    Other,
}

impl AppEvent for GlobalMsg {}


pub struct MyAppComponent;


impl AppComponent for MyAppComponent {
    type Msg = GlobalMsg;
    type State = GlobalState;

    fn render(&self, state: &Self::State, mut ctx: AppContext<Self::Msg>) -> AppNode<Self::Msg> {
        let callback = ctx.create_callback(|_s: &String| GlobalMsg::Other);
        let child = MyModuleComponent { callback: callback.into() };
        let node = child.render(&state.module_state,AppContext::new());

        AppNode::empty(ctx).with_child_and_converter(node, |e: ModuleLocalMsg| GlobalMsg::Local(e))
    }
}