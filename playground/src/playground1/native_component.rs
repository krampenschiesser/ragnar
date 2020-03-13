use crate::playground1::node::Node;

pub trait NativeEvent {}

pub trait NativeComponent {
    fn render(self) -> Node;
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