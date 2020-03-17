use std::borrow::Cow;

use crate::playground1::callback::Callback;

use crate::playground1::node::Node;

pub trait NativeEvent {}

pub trait NativeComponent: NativeComponentWrapper {
    fn render(self) -> Node;
    fn create_native_callback<T, In, Out>(name: T, callback: Box<dyn Fn(&In) -> Out>) -> Callback<In, Out>
        where T: Into<Cow<'static, str>>, In: NativeEvent {
        Callback::new_native(name, callback)
    }
}

pub trait NativeComponentWrapper {
    fn render(self) -> Node;
}

impl<T: NativeComponent> NativeComponentWrapper for T {
    fn render(self) -> Node {
        NativeComponent::render(self)
    }
}

// pub struct NativeComponent {
//     pub id: u32,
//     pub name: Cow<'static, str>,
//     pub attributes: HashMap<Cow<'static, str>,Cow<'static, str>>,
//     pub children: NativeComponentChildren,
// }
//
// pub enum NativeComponentChildren {
//     TextChild(Cow<'static, str>),
//     Children(Vec<NativeComponent>),
//     Empty,
// }