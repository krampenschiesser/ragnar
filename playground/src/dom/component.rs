use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::dom::node::Node;

pub trait Component {
    type Message;
    type State;

    fn update(&mut self, msg: Self::Message) -> bool;

    fn render(&self, context: &mut Context, state: Self::State) -> Node;
}


pub struct Context {
    counter: AtomicU64,
    components: HashMap<u64, Box<dyn Any>>,
    callbacks: HashMap<u64, Box<dyn Fn(Box<dyn Any>, Box<dyn Any>) -> Box<dyn Any>>>,
}

pub struct ComponentInContext<C: Component> {
    component: C,
    id: u64,
}

impl Context {
    pub fn on_event<Event: 'static>(&self, event_id: u64, event: Event) {
        if let Some(callback) = self.callbacks.get(&event_id) {
            // let component = self.components.get(&event_id).unwrap();

            // let msg = (callback)(Box::new(event), Box::new(component));
        };
    }
    pub fn register_local_callback<State, Comp: Component<Message=Msg, State=State> + 'static, Msg, Event: 'static, Func: 'static>(&mut self, component: &Comp, func: Func)
        where Func: Fn(&Event) -> Msg {
        let new_id = self.counter.fetch_add(1, Ordering::AcqRel);
        // self.callbacks.insert(new_id, Box::new(move |event: Box<dyn Any>, component: Box<dyn Any>| {
        //     let ev: &Event = event.downcast_ref().unwrap();
        //     let msg = func(ev);
        //     let comp: &mut Comp = component.downcast_mut().unwrap();
        //     comp.update(msg);
        //     Box::new(String::new())
        // }));
    }
}

pub trait RenderAble {
    fn render<S, T>(context: Context, state: S) -> Node;
}


#[cfg(test)]
mod tests {
    use anymap::AnyMap;

    use crate::dom::node::Listener;

    use super::*;

    struct Counter {
        count: u32,
    }

    struct GlobalState {
        msg: String
    }

    struct LocalMsg;

    struct GlobalMsg;

    impl Component for Counter {
        type Message = GlobalMsg;
        type State = GlobalState;

        fn update(&mut self, msg: Self::Message) -> bool {
            unimplemented!()
        }

        fn render(&self, context: &mut Context, state: Self::State) -> Node {
            context.register_local_callback(self, |_: &String| GlobalMsg);

            // let wrapper =  Box::new(|map: anymap::AnyMap| {
            //     let mystring = map.get::<String>().unwrap();
            //     println!("{}, {}", mystring, self.count);
            // });
            // let listener = Listener {
            //     event: "bla".into(),
            //     callback: wrapper,
            // };
            Node {
                name: "Button".into(),
                children: Vec::new(),
                attributes: Vec::new(),
                listeners: Vec::new(),
            }
        }
    }
}
