#[derive(Default,Debug)]
pub struct DomElement {
    children: Vec<Box<DomElement>>
}

pub trait Component {
    type GlobalState;
    type LocalState;
    fn filter_state(state: &Self::GlobalState) -> &Self::LocalState;
    fn render(&self) -> DomElement;
}
