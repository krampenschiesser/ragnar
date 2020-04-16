#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate ragnar_rsx_macro;
#[macro_use]
extern crate ragnar_derive_component_builder;

use ragnar_lib::{AppState, AppEvent};



pub mod state;
pub mod component;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
