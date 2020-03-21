

use crate::playground1::app_component::{AppComponent, AppEvent, AppState};
use crate::playground1::callback::TypedInputCallbackRef;
use crate::playground1::example::module::module1::{ModuleLocalMsg, ModuleLocalState, MyModuleComponent};
use crate::playground1::module::Module;

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


pub struct MyModule {
    callback: TypedInputCallbackRef<String>,
}

impl Module for MyModule {
    type Msg = ModuleLocalMsg;
    type ParentMsg = GlobalMsg;
    type State = ModuleLocalState;
    type ParentState = GlobalState;
    type Component = MyModuleComponent;

    fn convert_state(state: &Self::ParentState) -> &Self::State {
        &state.module_state
    }

    fn convert_event(event: Self::Msg) -> Self::ParentMsg {
        GlobalMsg::Local(event)
    }

    fn get_app_component(_state: &Self::State) -> Self::Component {
        // MyModuleComponent {callback:self.ca}
        unimplemented!()
    }
}

pub struct Mod2;

impl AppComponent for Mod2 {
    type Msg = ModuleLocalMsg;
    type State = ModuleLocalState;

    fn render(&self, _state: &Self::State) -> AppNode<Self::Msg> {
        unimplemented!()
    }
}