#![feature(vec_remove_item)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use std::sync::atomic::{AtomicU64, Ordering};

mod callback;
mod local_component;
mod app_component;
mod native_component;
mod node;
mod runtime;
mod attribute;
mod example;
mod component;

mod prelude;
pub use prelude::*;


pub(crate) static INCREMENTER: Incrementer = Incrementer { counter: AtomicU64::new(0) };

pub(crate) struct Incrementer {
    counter: AtomicU64,
}

impl Incrementer {
    pub fn get_next(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::AcqRel)
    }
}