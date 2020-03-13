// use serde::{Serialize, de::DeserializeOwned};
// use std::sync::Arc;

// mod dom;
// mod module1;
mod playground1;

/**
pub use dom::DomElement;
use serde::export::fmt::Debug;


pub enum UpdateResult {
    ShouldRender,
    SkipRender,
}

pub trait GlobalRender{}
pub struct Renderable{}

pub trait Component<MSG: Message, STATE: State<MSG>> {
    // fn update(&mut self, state: &STATE) -> UpdateResult {
    //     UpdateResult::ShouldRender
    // };
    fn render(&self, state: &STATE) -> Renderable;
}

pub trait Message: Serialize + DeserializeOwned + Clone+Debug {}

pub trait State<T: Message>: Serialize + DeserializeOwned + Clone + Default {
    fn update(&mut self, message: T);
}

pub struct Module<M: Message, S: State<M>> {
    initial_state: S,
    current_state: S,
    messages: Vec<M>,
}

pub struct ChildModule<MI: Message, M: Message, SI: State<MI>, S: State<M>> {
    module: Module<M, S>,
    inherited_state: Option<Arc<SI>>,
    inherited_message_callback: Option<Box<dyn Fn(MI)>>,
}

impl<M: Message, S: State<M>> Module<M, S> {
    pub fn from_json(initial_state_json: &str, current_state_json: &str, messages_json: &str) -> ::serde_json::Result<Self> {
        let result = Self::deserialize_messages(messages_json);
        match result {
            Ok(vec) => Self::initialize(initial_state_json, vec),
            Err(e) => {
                log::error!("Could not deserialize messages, maybe definition changed? will skip and use accumulated state. Error: {}", e);
                Self::initialize(current_state_json, Vec::new())
            }
        }
    }

    pub fn deserialize_messages(json: &str) -> ::serde_json::Result<Vec<M>> {
        serde_json::from_str(json)
    }

    fn initialize(state: &str, messages: Vec<M>) -> ::serde_json::Result<Self> {
        let initial_state: S = ::serde_json::from_str(state)?;

        let mut current_state = initial_state.clone();
        for message in &messages {
            current_state.update(message.clone());
        }

        Ok(Self {
            initial_state,
            current_state,
            messages,
        })
    }

    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            current_state: S::default(),
            initial_state: S::default(),
        }
    }
}

impl<M: Message, S: State<M>> Default for Module<M, S> {
    fn default() -> Self {
        Self::new()
    }
}

*/
struct Bla;