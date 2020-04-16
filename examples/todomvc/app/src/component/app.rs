use ragnar_lib::{AppComponent, AppNode, AppContext};
use crate::state::{Msg, State};

pub struct App {

}

impl AppComponent for App {
    type Msg = Msg;
    type State = State;

    fn render(&self, state: &Self::State, ctx: AppContext<Self::Msg>) -> AppNode<Self::Msg> {

        app!(
            <div>

            </div>
        );
        unimplemented!()
    }
}