use ragnar_lib::{NativeComponent, NativeNode, Node, NativeContext};
use crate::global::{GlobalAttributes, GlobalCallbacks, NativeApply};

#[derive(Component,Default)]
pub struct Li {
    pub children: Vec<Node>,

    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for Li {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("li",ctx)
            .with_children(self.children);

        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
