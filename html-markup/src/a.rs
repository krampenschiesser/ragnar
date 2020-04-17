use std::borrow::Cow;
use ragnar_lib::{ NativeComponent,NativeContext, NativeNode, Node};
use crate::global::{GlobalAttributes, GlobalCallbacks, NativeApply};

#[derive(Component, Default)]
pub struct A {
    pub download: Option<bool>,
    pub href: Option<Cow<'static, str>>,
    #[rename("hreflang")]
    pub href_lang: Option<Cow<'static, str>>,
    pub ping: Option<Cow<'static, str>>,
    #[rename("referrerpolicy")]
    pub referrer_policy: Option<Cow<'static, str>>,
    pub rel: Option<Cow<'static, str>>,
    pub target: Option<Cow<'static, str>>,
    #[rename("type")]
    pub mime_type: Option<Cow<'static, str>>,

    pub children: Vec<Node>,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for A {
    fn render(self, ctx: NativeContext) -> NativeNode {
        let node = NativeNode::new("a",ctx)
            .set_if("download", self.download)
            .set_if("href", self.href)
            .set_if("hreflang", self.href_lang)
            .set_if("ping", self.ping)
            .set_if("referrerpolicy", self.referrer_policy)
            .set_if("rel", self.rel)
            .set_if("target", self.target)
            .set_if("type", self.mime_type)
            .with_children(self.children);
        let node = self.global_attributes.apply(node);
        let node = self.global_callbacks.apply(node);
        node
    }
}
