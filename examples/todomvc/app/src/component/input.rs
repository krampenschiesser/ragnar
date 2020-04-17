use ragnar_lib::{AppComponent, AppNode, AppContext};
use crate::state::{Msg, State};
use ragnar_html_markup::event::{InputEvent, KeyboardEvent};

pub struct Input;


impl AppComponent for Input {
    type Msg = Msg;
    type State = State;

    fn render(&self, state: &Self::State, mut ctx: AppContext<Self::Msg>) -> AppNode<Self::Msg> {
        let inputcallback = ctx.create_callback(|e: &InputEvent| Msg::Update(e.data.clone()));
        let keycallback = ctx.create_callback(|e: &KeyboardEvent| {
            if e.key == "Enter" { Msg::Add } else { Msg::Nope }
        });
        app! {
            <InputText class="new-todo"
                placeholder="What needs to be done?"
                value={state.value.clone()}
                oninput={inputcallback}
                onkeypress={keycallback} />
        }
    }
}