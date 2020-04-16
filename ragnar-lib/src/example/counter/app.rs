use crate::app_component::{AppComponent, AppEvent, AppState, AppContext};
use crate::example::counter::local::IncDecWidget;
use crate::example::counter::native::Label;
use crate::local_component::{LocalComponent, LocalContext};

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

    fn render(&self, state: &Self::State, mut ctx: AppContext<Self::Msg>) -> AppNode<Self::Msg> {
        let _label = Label { text: format!("Clicked: {}", state.count).into() };

        let callback = ctx.create_callback(Box::new(|value: &u32| {
            StateChange::NewCount(*value)
        }));

        let widget = IncDecWidget {
            count: state.count,
            on_change: callback.into(),
        };

        AppNode::empty(ctx).with_child(widget.render(LocalContext::new())).into()
    }
}