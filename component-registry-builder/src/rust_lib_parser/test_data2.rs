
use std::borrow::Cow;
use std::include;

use ragnar_lib::{NativeCallback, NativeComponent, NativeEvent, NativeNode, TypedInputCallbackRef,Node};

use crate::event::MouseEvent;
use crate::form::FormId;
use crate::global::ReferenceId;
use crate::css::{CssStyle, CssClass};

#[derive(Component)]
pub struct Li {
    pub style: Option<CssStyle>,
    pub class: Option<CssClass>,
    pub children: Vec<Node>,

    //GLOBAL_START
    pub on_click: Option<TypedInputCallbackRef<MouseEvent>>,
    //GLOBAL_END
}

impl NativeComponent for Li {
    fn render(self, ctx: NativeContext) -> NativeNode {
        NativeNode::new("li")
            .set_if("style",self.style)
            .set_if("class",self.class)
            .with_children(self.children)
            .with_callback_if("onclick",self.on_click)
    }
}
