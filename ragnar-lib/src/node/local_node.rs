use crate::node::{NodeId, Node};
use crate::local_component::{LocalComponentWrapper, LocalContext};
use crate::callback::LocalCallbackWrapper;
use crate::{INCREMENTER, LocalEvent};

pub struct LocalNode {
    pub id: NodeId,
    pub component: Box<dyn LocalComponentWrapper>,
    pub children: Option<Vec<Node>>,
    pub callbacks: Vec<LocalCallbackWrapper>,
}


impl LocalNode {
    pub fn new<C: 'static + LocalComponentWrapper,U: LocalEvent>(component: C, ctx: LocalContext<U>) -> Self {
        let id = NodeId(INCREMENTER.get_next());
        let callbacks = ctx.into_callbacks().into_iter().map(|mut c|{
            c.node_id = id;
            c
        }).collect();
        LocalNode {
            id,
            component: Box::new(component),
            children: None,
            callbacks,
        }
    }


    pub fn with_children<T: Into<Node>>(mut self, nodes: Vec<T>) -> Self {
        for node in nodes {
            self = self.with_child(node);
        }
        self
    }

    pub fn with_child<T: Into<Node>>(mut self, node: T) -> Self {
        if self.children.is_none() {
            self.children = Some(Vec::with_capacity(1));
        }
        if let Some(children) = &mut self.children {
            children.push(node.into());
        }
        self
    }
}

impl Into<Node> for LocalNode {
    fn into(self) -> Node {
        Node::Local(self)
    }
}