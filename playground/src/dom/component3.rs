use downcast_rs::{Downcast, impl_downcast};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::ops::{Deref, Index};
use std::rc::Rc;

use crate::dom::node::{Attribute, Listener};

pub struct Node {
    pub name: String,
    pub listeners: Vec<Listener>,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
    pub state: Box<dyn ComponentHandler>,
}

impl Node {
    pub fn new<T: 'static + ComponentHandler>(state: T) -> Self {
        Self {
            name: "".to_string(),
            listeners: Vec::new(),
            attributes: Vec::new(),
            children: Vec::new(),
            state: Box::new(state),
        }
    }
}

pub trait Component {
    type Msg;
    type State;
    // fn render<'a, 'bump: 'a>(&self, context: &'a mut Context, bump: &'bump bumpalo::Bump);

    fn render<C: Component<Msg=Self::Msg, State=Self::State> + 'static>(this: RegisteredComponent<C>, state: &Self::State) -> Node;

    fn update(&self, msg: &Self::Msg) -> Self;


    fn register_component<C: Component>(component: C) -> RegisteredComponent<C> {
        RegisteredComponent {
            id: 0,
            callback_counter: 0,
            component,
            callbacks: HashMap::new(),
        }
    }
}

pub struct RegisteredComponent<C: Component> {
    id: u64,
    callback_counter: u64,
    component: C,
    callbacks: HashMap<u64, Box<dyn Fn(Box<dyn Any>) -> Option<C::Msg>>>,
}

impl<C: Component + 'static> RegisteredComponent<C> {
    fn add_callback<T: UiEvent + 'static>(&mut self, callback: Box<dyn  Fn(&T) -> C::Msg>) {
        let mycall = Box::new(move |t: Box<dyn Any>| {
            if let Some(event) = t.downcast_ref::<T>() {
                let msg = callback(event);
                return Some(msg);
            }
            return None;
        });
        self.callbacks.insert(self.callback_counter, mycall);
        self.callback_counter += 1;
    }

    fn render(self, state: &C::State) -> Node {
        C::render(self, state)
    }
}

// impl<C: Component> Deref for RegisteredComponent<C> {
//     type Target = C;
//
//     fn deref(&self) -> &Self::Target {
//         &self.component
//     }
// }

trait UiEvent {}

impl UiEvent for String {}

pub trait ComponentHandler {
    fn handle(&mut self, callback_id: u64, e: Box<dyn UiEvent>);
}

impl<C: Component> ComponentHandler for RegisteredComponent<C> {
    fn handle(&mut self, callback_id: u64, e: Box<dyn UiEvent>) {
        let x = self.callbacks.get(&callback_id).unwrap();
        let option = (x)(Box::new(e));
        if let Some(msg) = option {
            self.component.update(&msg);
        }
    }
}


#[cfg(test)]
mod tests {
    use bumpalo::Bump;
    use std::borrow::BorrowMut;

    use super::*;

    struct MyMsg;

    struct MyState {
        child: ChildState,
    }

    struct ChildState;

    struct Other(u32);

    impl Component for Other {
        type Msg = MyMsg;
        type State = ChildState;

        fn render<C: 'static + Component<Msg=Self::Msg, State=Self::State>>(this: RegisteredComponent<C>, state: &Self::State) -> Node {
            let x = Box::new(|e: &String| {
                MyMsg
            });
            Node::new(this)
        }
        //
        // fn render(&self, state: &Self::State) -> Node {
        // }

        fn update(&self, msg: &Self::Msg) -> Self{
            Other(self.0+1)
        }
    }

    struct Bla(u32);


    impl Component for Bla {
        type Msg = MyMsg;
        type State = MyState;

        fn render<C: 'static + Component<Msg=Self::Msg, State=Self::State>>(mut this: RegisteredComponent<C>, state: &Self::State) -> Node {
            let x = Box::new(|e: &String| -> MyMsg {
                let ret = MyMsg;
                ret
            });
            this.add_callback(x);
            let mut child = Self::register_component(Other(0));
            let child_node = child.render(&state.child);
            let mut node = Node::new(this);
            node.children.push(child_node);
            node
        }

        fn update(&self, msg: &Self::Msg) -> Self {
            Bla(self.0+1)
        }
    }

    #[test]
    fn test_event() {
        let state = MyState { child: ChildState };

        let c = Bla(0);
        let mut component = Bla::register_component(c);
        let mut node = component.render(&state);

        node.state.handle(0, Box::new(String::from("bla")));

        // assert_eq!(component.component.0, 1u32);

        //
        // let node = bla.render(&context, &state);
        // context.create_component(0, bla);
        // {
        //     context.on_event(0, 0, Box::new(String::from("Hello")));
        //     context.with_component(0, |b: &Bla| {
        //         assert_eq!(2, b.0);
        //     });
        //     context.with_component(1, |b: &Other| {
        //         assert_eq!(3, b.0);
        //     });
        // }
        // context.on_event(1, 1, Box::new(String::from("Hello")));
        //
        // context.with_component(1, |b: &Other| {
        //     assert_eq!(3, b.0);
        // });
    }
}