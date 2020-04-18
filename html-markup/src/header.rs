use ragnar_lib::{NativeComponent, NativeContext, NativeNode, Node};
use crate::global::{GlobalCallbacks, GlobalAttributes, NativeApply};

#[derive(Component, Default)]
pub struct Header {
    pub children: Vec<Node>,

    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for Header {
    fn render(self, ctx: NativeContext) -> NativeNode {
        impl_basic!("header",self,ctx)
    }
}
