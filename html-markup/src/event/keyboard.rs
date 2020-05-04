use crate::event::Event;
use ragnar_lib::NativeEvent;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeyboardEvent {
    pub alt_key: bool,
    pub code: String,
    pub ctrl_key: bool,
    pub is_composing: bool,
    pub key: String,
    pub locale: String,
    pub location: u16,
    pub meta_key: bool,
    pub repeat: bool,
    pub shift_key: bool,
    pub event: Event,
}

impl NativeEvent for KeyboardEvent {
    fn get_type() -> &'static str where Self: Sized {
        "html.keyboardevent"
    }
}