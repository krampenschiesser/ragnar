use ragnar_lib::{AppComponent, AppNode, AppCallback, AppContext};
use crate::state::{Msg, State, Filter};
use ragnar_html_markup::event::MouseEvent;

#[derive(Component)]
pub struct FilterView {
    #[required]
    pub filter: Filter,
}

impl AppComponent for FilterView {
    type Msg = Msg;
    type State = State;

    fn render(&self, state: &Self::State, mut ctx: AppContext<Self::Msg>) -> AppNode<Self::Msg> {
        let filter = self.filter.clone();
        let callback = ctx.create_callback(move |click: &MouseEvent| Msg::SetFilter(filter));
        app! {
            <li>
                <a class={if state.filter == self.filter { "selected" } else { "not-selected" }}
                   onclick={callback}>
                    { self.filter.to_string() }
                </a>
            </li>
        }
    }
}

mod test {
    struct Test { string: String }

    impl Test {
        fn calls(&self) -> Box<dyn FnOnce(&str) -> String> {
            let first_clone = self.string.clone();
            Box::new(move |_: &str| { first_clone })
        }
    }
}