use crate::playground1::app_component::{AppComponent, AppEvent, AppState};
use crate::playground1::example::local::IncDecWidget;
use crate::playground1::example::native::{Div, Label};
use crate::playground1::local_component::LocalComponent;
use crate::playground1::node::{Node, NodeChildren, NodeComponentWrapper};

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

    fn render(&self, state: &Self::State) -> Node {
        let label = Label { text: format!("Clicked: {}", state.count).into() };

        let callback = Self::create_app_callback(Box::new(|value: &u32| {
            StateChange::NewCount(*value)
        }));

        let widget = IncDecWidget {
            count: state.count,
            on_change: callback.get_input_ref(),
        };
        Node {
            component: NodeComponentWrapper::None,
            callbacks: vec![callback.into()],
            native_name: None,
            children: NodeChildren::Nodes(vec![widget.render()]),
        }
    }
}