use crate::{Node, TextNode};


pub trait ExtendNodeChildren {
    fn extend_children(self, children: &mut Vec<Node>);
}

impl ExtendNodeChildren for Node {
    fn extend_children(self, children: &mut Vec<Node>) {
        children.push(self);
    }
}

impl ExtendNodeChildren for String {
    fn extend_children(self, children: &mut Vec<Node>) {
        children.push(Node::Text(TextNode::new(self)));
    }
}

impl ExtendNodeChildren for &'_ str {
    fn extend_children(self, children: &mut Vec<Node>) {
        children.push(Node::Text(TextNode::new(self.to_string())));
    }
}

impl ExtendNodeChildren for Vec<Node> {
    fn extend_children(self, children: &mut Vec<Node>) {
        children.extend(self.into_iter());
    }
}

impl ExtendNodeChildren for std::vec::IntoIter<Node> {
    fn extend_children(self, children: &mut Vec<Node>) {
        children.extend(self);
    }
}

impl ExtendNodeChildren for std::collections::hash_set::IntoIter<Node> {
    fn extend_children(self, children: &mut Vec<Node>) {
        children.extend(self);
    }
}

impl ExtendNodeChildren for std::collections::linked_list::IntoIter<Node> {
    fn extend_children(self, children: &mut Vec<Node>) {
        children.extend(self);
    }
}

impl ExtendNodeChildren for std::collections::btree_set::IntoIter<Node> {
    fn extend_children(self, children: &mut Vec<Node>) {
        children.extend(self);
    }
}

impl ExtendNodeChildren for std::collections::vec_deque::IntoIter<Node> {
    fn extend_children(self, children: &mut Vec<Node>) {
        children.extend(self);
    }
}
impl<I: std::iter::Iterator, F:  FnMut(<I as Iterator>::Item) -> Node> ExtendNodeChildren for std::iter::Map<I,F> {
    fn extend_children(self, children: &mut Vec<Node>) {
        let vec: Vec<_> = self.collect();
        children.extend(vec);
    }
}

macro_rules! impl_format {
    ($i:ty) => {
        impl ExtendNodeChildren for $i {
            fn extend_children(self, children: &mut Vec<Node>) {
                children.push(Node::Text(TextNode::new(format!("{}", self))));
            }
        }
    }
}
impl_format!(usize);
impl_format!(u8);
impl_format!(u16);
impl_format!(u32);
impl_format!(u64);
impl_format!(u128);
impl_format!(isize);
impl_format!(i8);
impl_format!(i16);
impl_format!(i32);
impl_format!(i64);
impl_format!(i128);
impl_format!(f32);
impl_format!(f64);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TextNode;

    #[test]
    fn test_extend() {
        use ExtendNodeChildren;
        let mut children = Vec::new();

        let new_node = || { Node::Text(TextNode::new("bla")) };

        ExtendNodeChildren::extend_children(new_node(), &mut children);

        ExtendNodeChildren::extend_children(vec![new_node(), new_node()].into_iter(), &mut children);
        ExtendNodeChildren::extend_children(vec![new_node(), new_node()], &mut children);
        assert_eq!(5, children.len());
    }
}