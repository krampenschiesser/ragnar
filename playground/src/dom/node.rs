use std::any::Any;

pub struct Listener {
    pub event: String,
    pub callback: Box<dyn Fn(Box<dyn Any>)>,
}

pub enum Attribute {
    Bool(bool),
    Text(String),
    Double(f64),
    Unsigned(u64),
    Signed(i64),
}

pub struct Node {
    pub name: String,
    pub listeners: Vec<Listener>,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

impl Default for Node {
    fn default() -> Self {
        Node{
            name: "".into(),
            listeners: Vec::new(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct OnString {}

    #[test]
    fn run() {
        let wrapper = Box::new(|any: Box<dyn Any>| {
            let mystring: &String = any.downcast_ref().unwrap();
            println!("{}", mystring);
        });
        let listener = Listener {
            event: "bla".into(),
            callback: wrapper,
        };

        let x = Box::new(String::from("bla"));
        (listener.callback)(x);
    }
}
