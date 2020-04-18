#[macro_use]
extern crate ragnar_lib;

#[macro_use]
pub mod mac {
    macro_rules! impl_basic {
        ($i:expr,$s:ident,$ctx:ident) => {
            {
               let node = NativeNode::new($i,$ctx)
                    .with_children($s.children);

                let node = $s.global_attributes.apply(node);
                let node = $s.global_callbacks.apply(node);
                node
            }
        }
    }
}

pub mod a;
pub mod div;
pub mod button;
pub mod label;
pub mod event;
pub mod global;
pub mod css;
pub mod form;
pub mod ul;
pub mod ol;
pub mod li;
pub mod input;
pub mod section;
pub mod header;
pub mod footer;
pub mod span;
pub mod strong;
pub mod p;
pub mod h;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
