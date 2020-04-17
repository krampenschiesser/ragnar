use std::borrow::Cow;
use ragnar_lib::{NativeComponent, NativeContext, NativeNode};
use crate::global::{GlobalAttributes, GlobalCallbacks, NativeApply};
use crate::input::{FormInputAttributes, CommonInputAttributes};

#[derive(Component, Default)]
pub struct InputImage {
    pub alt: Option<Cow<'static, str>>,
    pub src: Option<Cow<'static, str>>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    #[delegated]
    pub form_attributes: FormInputAttributes,
    #[delegated]
    pub common_input_attributes: CommonInputAttributes,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for InputImage {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("input",ctx)
            .set("type","image")
            .set_if("alt",self.alt)
            .set_if("src",self.src)
            .set_if("width",self.width)
            .set_if("height",self.height)
            ;

        let node = self.common_input_attributes.apply(node);
        let node = self.form_attributes.apply(node);
        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
