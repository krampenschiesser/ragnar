use crate::callback::TypedInputCallbackRef;
use crate::example::counter::native::{Button, ClickEvent, Div};
use crate::local_component::{LocalComponent, LocalEvent, UpdateResult, LocalContext};
use crate::native_component::{NativeComponent, NativeContext};

use crate::node::local_node::LocalNode;

pub struct IncDecWidget {
    pub count: u32,
    pub on_change: TypedInputCallbackRef<u32>,
}

pub enum IncDecMsg {
    Increment,
    Decrement,
}

impl LocalEvent for IncDecMsg {}

impl LocalComponent for IncDecWidget {
    type Msg = IncDecMsg;

    fn render(self, mut ctx: LocalContext<Self::Msg>) -> LocalNode {
        let increment_callback = ctx.create_callback(|_click: &ClickEvent| {
            IncDecMsg::Increment
        });
        let decrement_callback = ctx.create_callback(|_click: &ClickEvent| {
            IncDecMsg::Decrement
        });
        let increment = Button {
            title: "increment".into(),
            on_click: increment_callback.into(),
        };
        let decrement = Button {
            title: "decrement".into(),
            on_click: decrement_callback.into(),
        };

        let mut children = Vec::new();
        children.push(increment.render(NativeContext::new()).into());
        children.push(decrement.render(NativeContext::new()).into());
        let div = Div {
            children,
        };
        let div2 = Div { children: vec![] };
        LocalNode::new(self, ctx).with_child(div.render(NativeContext::new()))
    }


    fn update(&self, msg: &Self::Msg, ctx: LocalContext<Self::Msg>) -> UpdateResult<Self> {
        UpdateResult::NewState(Box::new(match msg {
            IncDecMsg::Increment => Self {
                on_change: self.on_change,
                count: self.count + 1,
            },
            IncDecMsg::Decrement => Self {
                on_change: self.on_change,
                count: self.count - 1,
            },
        }))
    }
}