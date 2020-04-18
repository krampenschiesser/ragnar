use ragnar_lib::{NativeComponent, NativeContext, NativeNode, Node};
use crate::global::{GlobalCallbacks, GlobalAttributes, NativeApply};


#[derive(Component, Default)]
pub struct Span {
    pub children: Vec<Node>,

    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for Span {
    fn render(self, ctx: NativeContext) -> NativeNode {
        impl_basic!("span",self,ctx)
    }
}
