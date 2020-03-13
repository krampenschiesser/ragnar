use downcast_rs::{Downcast, impl_downcast};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Index;
use std::rc::Rc;

use crate::dom::node::Node;

pub trait BaseComponent: Downcast {
    fn update_base(&mut self, event: Box<dyn Any>) {}
}
impl_downcast!(BaseComponent);

pub trait Component: BaseComponent {
    type Msg: 'static;
    type State;
    // fn render<'a, 'bump: 'a>(&self, context: &'a mut Context, bump: &'bump bumpalo::Bump);

    fn render(&self, context: &Context, state: &Self::State) -> Node;

    fn update(&mut self, msg: &Self::Msg);
}

impl<T: Component> BaseComponent for T {
    fn update_base(&mut self, event: Box<dyn Any>) {
        let x: &T::Msg = event.downcast_ref().unwrap();
        self.update(x);
    }
}

pub struct Context(RefCell<ContextInner>);

pub struct ContextInner {
    components: HashMap<u64, Box<dyn BaseComponent>>,
    callbacks: HashMap<u64, Box<dyn Fn(Box<dyn Any>) -> Box<dyn  Any>>>,
}

impl Context {
    pub fn on_event(&self, component_id: u64, callback_id: u64, event: Box<dyn Any>) {
        self.0.borrow_mut().on_event(component_id, callback_id, event)
    }
    pub fn register_callback<Event: 'static, Message: Any>(&self, component_id: u64, callback: Box<dyn Fn(&Event) -> Message>) {
        self.0.borrow_mut().register_callback(component_id, callback)
    }
    pub fn create_component<C: Component>(&self, id: u64, c: C) {
        self.0.borrow_mut().create_component(id, c)
    }

    pub fn with_component<C: Component, R, F: Fn(&C) -> R>(&self, id: u64, function: F) -> R {
        let x1 = self.0.borrow_mut();
        let x = x1.get_component::<C>(id);
        function(x)
    }
}

impl ContextInner {
    pub fn on_event(&mut self, component_id: u64, callback_id: u64, event: Box<dyn Any>) {
        let callback = self.callbacks.get_mut(&component_id).unwrap();
        let msg = (callback)(event);
        let component = self.components.get_mut(&component_id).unwrap();
        component.update_base(msg);
    }

    pub fn register_callback<Event: 'static, Message: Any>(&mut self, component_id: u64, callback: Box<dyn Fn(&Event) -> Message>) {
        let wrapped = Box::new(move |any_in: Box<dyn Any>| {
            let event: &Event = any_in.downcast_ref().unwrap();
            let message = (callback)(event);
            // let b: Box<dyn Any> = Box::new(message).into();
            // b
            Box::new(message) as Box<dyn Any>
        });
        self.callbacks.insert(component_id, wrapped);
    }

    pub fn create_component<C: Component>(&mut self, id: u64, c: C) {
        self.components.insert(id, Box::new(c));
    }

    pub fn get_component<C: Component>(&self, id: u64) -> &C {
        self.components.get(&id).unwrap().downcast_ref::<C>().unwrap()
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

        fn render(&self, context: &Context, state: &Self::State) -> Node {
            let x = Box::new(|e: &String| {
                MyMsg
            });
            context.register_callback(0, x);
            Node::default()
        }

        fn update(&mut self, msg: &Self::Msg) {
            self.0 = self.0 + 1;
        }
    }

    struct Bla(u32);


    impl Component for Bla {
        type Msg = MyMsg;
        type State = MyState;

        fn render(&self, context: &Context, state: &Self::State) -> Node {
            let x = Box::new(|e: &String| {
                MyMsg
            });
            context.register_callback(1, x);
            context.create_component(1, Other(0));
            let child_node = context.with_component(1, |o: &Other| {
                o.render(context, &state.child)
            });
            // unimplemented!()
            Node::default()
        }

        fn update(&mut self, msg: &Self::Msg) {
            self.0 = self.0 + 1;
        }
    }

    #[test]
    fn test_event() {
        let mut context = Context(RefCell::new(ContextInner {
            callbacks: HashMap::new(),
            components: HashMap::new(),
        }));
        let bla = Bla(1);

        let state = MyState { child: ChildState };
        let node = bla.render(&context, &state);
        context.create_component(0, bla);
        {
            context.on_event(0, 0, Box::new(String::from("Hello")));
            context.with_component(0, |b: &Bla| {
                assert_eq!(2, b.0);
            });
            context.with_component(1, |b: &Other| {
                assert_eq!(3, b.0);
            });
        }
        context.on_event(1, 1, Box::new(String::from("Hello")));

        context.with_component(1, |b: &Other| {
            assert_eq!(3, b.0);
        });
    }
}