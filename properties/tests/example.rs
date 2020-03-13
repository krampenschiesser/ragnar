extern crate properties;

use properties::Properties;


#[derive(Properties)]
struct PropertyExample {
    number1: u32,
    number2: isize,
    name: String,
    some: Option<String>,
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}