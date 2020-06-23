#![feature(vec_remove_item)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use std::sync::atomic::{AtomicU64, Ordering};

mod app_component;
mod attribute;
mod callback;
mod component;
mod example;
mod local_component;
mod native_component;
mod node;
mod runtime;

mod prelude;

pub use prelude::*;
use std::sync::Arc;

pub(crate) static INCREMENTER: Incrementer = Incrementer {
    counter: AtomicU64::new(0),
};

pub(crate) struct Incrementer {
    counter: AtomicU64,
}

impl Incrementer {
    pub fn get_next(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::AcqRel)
    }
}

#[derive(Clone)]
pub struct App<
    C: AppComponent<State = State, Msg = Msg> + Clone,
    State: AppState + Clone,
    Msg: AppEvent,
> {
    pub root_component: C,
    pub update_function: Arc<Box<dyn Fn(&mut State, &Msg) + Send + Sync + 'static>>,
    pub native_event_resolvers: Arc<
        Vec<
            Box<
                dyn Fn(&str, &str) -> Result<Option<Box<dyn NativeEvent>>, String>
                    + Send
                    + Sync
                    + 'static,
            >,
        >,
    >,
    pub initial_state: State,
}

impl<C: AppComponent<State = State, Msg = Msg> + Clone, State: AppState + Clone, Msg: AppEvent>
    App<C, State, Msg>
{
    pub fn new(
        state: State,
        root_component: C,
        update_function: Box<dyn Fn(&mut State, &Msg) + Send + Sync + 'static>,
        native_event_resolvers: Vec<
            Box<
                dyn Fn(&str, &str) -> Result<Option<Box<dyn NativeEvent>>, String>
                    + Send
                    + Sync
                    + 'static,
            >,
        >,
    ) -> Self {
        Self {
            initial_state: state,
            root_component,
            update_function: Arc::new(update_function),
            native_event_resolvers: Arc::new(native_event_resolvers),
        }
    }
}
