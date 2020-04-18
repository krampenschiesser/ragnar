use ragnar_lib::{NativeComponent, NativeContext, NativeNode, Node};
use crate::global::{GlobalCallbacks, GlobalAttributes, NativeApply};


#[derive(Component, Default)]
pub struct Strong {
    pub children: Vec<Node>,

    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for Strong {
    fn render(self, ctx: NativeContext) -> NativeNode {
        impl_basic!("strong",self,ctx)
    }
}
