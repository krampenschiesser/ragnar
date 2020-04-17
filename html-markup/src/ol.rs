use ragnar_lib::{ NativeContext, NativeComponent, NativeNode, Node};
use crate::global::{GlobalAttributes, GlobalCallbacks, NativeApply};

#[derive(Component)]
pub struct Ol {
    pub children: Vec<Node>,

    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for Ol {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("ul",ctx)
            .with_children(self.children);

        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
