use crate::playground1::app_component::{AppState, AppEvent, AppComponent};

pub trait Module {
    type Msg: AppEvent;
    type ParentMsg: AppEvent;
    type State: AppState;
    type ParentState: AppState;
    type Component: AppComponent<Msg=Self::Msg,State=Self::State>;

    fn convert_state(state: &Self::ParentState) -> &Self::State;

    fn convert_event(event: Self::Msg) -> Self::ParentMsg;

    fn get_app_component(state: &Self::State) -> Self::Component;
}