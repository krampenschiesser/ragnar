use ragnar_lib::{NativeComponent, NativeContext, NativeNode, Node};
use crate::form::FormId;
use crate::global::{ReferenceId, GlobalAttributes, GlobalCallbacks, NativeApply};

#[derive(Component,Default)]
pub struct Label {
    #[rename("for")]
    pub reference_id: Option<ReferenceId>,
    #[rename("form")]
    pub form_id: Option<FormId>,
    pub children: Vec<Node>,

    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for Label {
    fn render(self, ctx: NativeContext) -> NativeNode {
     let node=   NativeNode::new("label",ctx)
            .set_if("for", self.reference_id)
            .set_if("form", self.form_id)
            .with_children(self.children);


        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
