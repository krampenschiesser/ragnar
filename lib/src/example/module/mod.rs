

use crate::playground1::app_component::{AppComponent, AppEvent, AppState};
use crate::playground1::callback::TypedInputCallbackRef;
use crate::playground1::example::module::module1::{ModuleLocalMsg, ModuleLocalState, MyModuleComponent};

use crate::playground1::node::app_node::AppNode;

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

    fn render(&self, state: &Self::State) -> AppNode<Self::Msg> {
        let callback = Self::create_app_callback(Box::new(|_s: &String| GlobalMsg::Other));
        let child = MyModuleComponent { callback: callback.get_input_ref() };
        let node = child.render(&state.module_state);

        AppNode::empty().with_child_and_converter(node, |e: ModuleLocalMsg| GlobalMsg::Local(e))
    }
}