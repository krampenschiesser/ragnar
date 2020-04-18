use ragnar_lib::{LocalComponent, UpdateResult, LocalNode, NativeComponent, TextNode, LocalContext};
use crate::state::Task;
use ragnar_html_markup::li::*;
use ragnar_html_markup::div::Div;
use ragnar_html_markup::label::Label;
use ragnar_html_markup::button::Button;

#[derive(Component)]
pub struct TodoComponent {
    todo: Task,
}

pub trait VecExtend<T> {
    fn extend_vec(self, vec: &mut Vec<T>);
}

impl VecExtend<String> for String {
    fn extend_vec(self, vec: &mut Vec<String>) {
        vec.push(self);
    }
}

impl VecExtend<String> for Vec<String> {
    fn extend_vec(self, vec: &mut Vec<String>) {
        vec.extend(self.into_iter())
    }
}

impl LocalComponent for TodoComponent {
    type Msg = ();

    fn render(self, ctx: LocalContext<Self::Msg>) -> LocalNode {
        local! {
            <li>
                <div class="view">
                    <label for="huhu">
                        "bla hallo\"\" welt 123 false"
                        <button/>
                        "blubb"
                    </label>
                </div>
            </li>
        }
    }

    fn update(&self, _msg: &Self::Msg, ctx: LocalContext<Self::Msg>) -> UpdateResult<Self> {
        UpdateResult::Keep
    }
}
