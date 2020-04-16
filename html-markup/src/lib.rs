
#[macro_use]
extern crate ragnar_lib;

pub mod a;
pub mod div;
pub mod button;
pub mod label;
pub mod event;
pub mod global;
pub mod css;
pub mod form;
pub mod ul;
pub mod ol;
pub mod li;
pub mod input;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
