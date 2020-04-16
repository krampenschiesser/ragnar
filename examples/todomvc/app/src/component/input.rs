use ragnar_lib::{AppComponent, AppNode, AppContext};
use crate::state::{Msg, State};

pub struct Input;


impl AppComponent for Input {
    type Msg = Msg;
    type State = State;

    fn render(&self, state: &Self::State, ctx: AppContext<Self::Msg>) -> AppNode<Self::Msg> {
        // Self::create_app_callback(|e)
        // app!{
        //     <input class="new-todo"
        //         placeholder="What needs to be done?"
        //         value={&self.state.value}
        //         oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
        //         onkeypress=self.link.callback(|e: KeyboardEvent| {
        //                if e.key() == "Enter" { Msg::Add } else { Msg::Nope }
        //         }) />
        // }
        unimplemented!()
    }
}