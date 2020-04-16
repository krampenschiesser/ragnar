use std::borrow::Cow;
use crate::{NodeId, INCREMENTER, Node};

pub struct TextNode {
    pub text: Cow<'static, str>,
    pub id: NodeId,
}

impl TextNode {
    pub fn new<T: Into<Cow<'static, str>>>(text: T) -> Self {
        TextNode {
            id: NodeId(INCREMENTER.get_next()),
            text: text.into(),
        }
    }
}


impl Into<Node> for TextNode {
    fn into(self) -> Node {
        Node::Text(self)
    }
}