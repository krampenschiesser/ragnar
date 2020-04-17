use ragnar_lib::{NativeComponent, NativeContext, NativeNode, Node};
use crate::global::{GlobalAttributes, GlobalCallbacks, NativeApply};


#[derive(Component,Default)]
pub struct Button {
    pub children: Vec<Node>,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for Button {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("button",ctx)
            .with_children(self.children);
        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
