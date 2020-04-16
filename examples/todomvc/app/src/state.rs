use ragnar_lib::{AppState, AppEvent};

pub struct State {
    pub entries: Vec<Task>,
    pub filter: Filter,
    pub value: String,
    pub edit_value: String,
}

impl AppState for State {}

pub struct Task {
    name: String,
    completed: bool,
    editing: bool,
}


pub enum Msg {
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Nope,
}

impl AppEvent for Msg {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl ToString for Filter {
    fn to_string(&self) -> String {
        match self {
            Filter::All => "All".into(),
            Filter::Active => "Active".into(),
            Filter::Completed => "Completed".into(),
        }
    }
}