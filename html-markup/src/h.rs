use ragnar_lib::{NativeComponent, NativeContext, NativeNode, Node};
use crate::global::{GlobalCallbacks, GlobalAttributes, NativeApply};


#[derive(Component, Default)]
pub struct H1 {
    pub children: Vec<Node>,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

#[derive(Component, Default)]
pub struct H2 {
    pub children: Vec<Node>,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

#[derive(Component, Default)]
pub struct H3 {
    pub children: Vec<Node>,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

#[derive(Component, Default)]
pub struct H4 {
    pub children: Vec<Node>,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

#[derive(Component, Default)]
pub struct H5 {
    pub children: Vec<Node>,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

#[derive(Component, Default)]
pub struct H6 {
    pub children: Vec<Node>,
    #[delegated]
    pub global_attributes: GlobalAttributes,
    #[delegated]
    pub global_callbacks: GlobalCallbacks,
}

impl NativeComponent for H1 { fn render(self, ctx: NativeContext) -> NativeNode { impl_basic!("h1",self,ctx) } }
impl NativeComponent for H2 { fn render(self, ctx: NativeContext) -> NativeNode { impl_basic!("h2",self,ctx) } }
impl NativeComponent for H3 { fn render(self, ctx: NativeContext) -> NativeNode { impl_basic!("h3",self,ctx) } }
impl NativeComponent for H4 { fn render(self, ctx: NativeContext) -> NativeNode { impl_basic!("h4",self,ctx) } }
impl NativeComponent for H5 { fn render(self, ctx: NativeContext) -> NativeNode { impl_basic!("h5",self,ctx) } }
impl NativeComponent for H6 { fn render(self, ctx: NativeContext) -> NativeNode { impl_basic!("h6",self,ctx) } }
