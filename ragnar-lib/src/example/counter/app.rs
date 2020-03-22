use crate::app_component::{AppComponent, AppEvent, AppState};
use crate::example::counter::local::IncDecWidget;
use crate::example::counter::native::Label;
use crate::local_component::LocalComponent;

use crate::node::app_node::AppNode;

pub struct State {
    count: u32,
}

impl AppState for State {}

pub enum StateChange {
    NewCount(u32),
}

impl AppEvent for StateChange {}

pub struct App {
    title: String,
    count: u32,
}

impl AppComponent for App {
    type Msg = StateChange;
    type State = State;

    fn render(&self, state: &Self::State) -> AppNode<Self::Msg> {
        let _label = Label { text: format!("Clicked: {}", state.count).into() };

        let callback = Self::create_app_callback(Box::new(|value: &u32| {
            StateChange::NewCount(*value)
        }));

        let widget = IncDecWidget {
            count: state.count,
            on_change: callback.get_input_ref(),
        };

        AppNode::empty().with_callback(callback).with_child(widget.render()).into()
    }
}