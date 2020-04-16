use crate::app_component::{AppComponent, AppEvent, AppState};
use crate::local_component::LocalComponent;
use crate::node::app_node::AppNode;

mod first_optional;
mod multiple_optional;
mod first_required;
mod mixed;

pub struct State {}

impl AppState for State {}

pub enum StateChange {}

impl AppEvent for StateChange {}

//
// impl AppComponent for App {
//     type Msg = StateChange;
//     type State = State;
//
//     fn render(&self, state: &Self::State) -> AppNode<Self::Msg> {
//         AppNode::empty()
//     }
// }