#[macro_use]
extern crate ragnar_lib;

use crate::event::{DataTransfer, FocusEvent, InputEvent, KeyboardEvent, MouseEvent};
use ragnar_lib::NativeEvent;

#[macro_use]
pub mod mac {
    macro_rules! impl_basic {
        ($i:expr,$s:ident,$ctx:ident) => {{
            let node = NativeNode::new($i, $ctx).with_children($s.children);

            let node = $s.global_attributes.apply(node);
            let node = $s.global_callbacks.apply(node);
            node
        }};
    }
}

pub mod a;
pub mod button;
pub mod css;
pub mod div;
pub mod event;
pub mod footer;
pub mod form;
pub mod global;
pub mod h;
pub mod header;
pub mod input;
pub mod label;
pub mod li;
pub mod ol;
pub mod p;
pub mod section;
pub mod span;
pub mod strong;
pub mod ul;

pub fn resolve_native_event(
    event_type: &str,
    payload: &str,
) -> Result<Option<Box<dyn NativeEvent>>, String> {
    if let Some(event) = deserialize::<DataTransfer>(event_type, payload)? {
        return Ok(Some(Box::new(event)));
    }
    if let Some(event) = deserialize::<FocusEvent>(event_type, payload)? {
        return Ok(Some(Box::new(event)));
    }
    if let Some(event) = deserialize::<InputEvent>(event_type, payload)? {
        return Ok(Some(Box::new(event)));
    }
    if let Some(event) = deserialize::<KeyboardEvent>(event_type, payload)? {
        return Ok(Some(Box::new(event)));
    }
    if let Some(event) = deserialize::<MouseEvent>(event_type, payload)? {
        return Ok(Some(Box::new(event)));
    }
    Ok(None)
}

fn deserialize<T: NativeEvent + serde::de::DeserializeOwned>(
    event_type: &str,
    payload: &str,
) -> Result<Option<T>, String> {
    let type_name = T::get_type();
    if type_name == event_type {
        let t: T = serde_json::from_str(payload).map_err(|e| format!("{}", e))?;
        Ok(Some(t))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
