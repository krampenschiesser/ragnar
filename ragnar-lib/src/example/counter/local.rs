use crate::callback::TypedInputCallbackRef;
use crate::example::counter::native::{Button, ClickEvent, Div};
use crate::local_component::{LocalComponent, LocalEvent, UpdateResult};
use crate::native_component::NativeComponent;

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

    fn render(self) -> LocalNode {
        let increment_callback = Self::create_local_callback(Box::new(|_click: &ClickEvent| {
            IncDecMsg::Increment
        }));
        let decrement_callback = Self::create_local_callback(Box::new(|_click: &ClickEvent| {
            IncDecMsg::Decrement
        }));
        let increment = Button {
            title: "increment".into(),
            on_click: increment_callback.get_input_ref(),
        };
        let decrement = Button {
            title: "decrement".into(),
            on_click: decrement_callback.get_input_ref(),
        };

        let mut children = Vec::new();
        children.push(increment.render().into());
        children.push(decrement.render().into());
        let div = Div {
            children,
        };
        LocalNode::new(self).with_child(div.render()).with_callback(increment_callback).with_callback(decrement_callback)

        //<div>
        //  <button on_click={|_click: &ClickEvent| IncDecMsg:::Increment}>increment</button>
        //  <button on_click={|_click: &ClickEvent| IncDecMsg:::Decrement}>{format!("{}","decrement")}</button>
        //</div>
        //
        //
        //
        //
        //
        //
        //
    }


    fn update(&self, msg: &Self::Msg) -> UpdateResult<Self> {
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