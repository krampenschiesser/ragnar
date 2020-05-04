use ragnar_lib::NativeEvent;
use crate::event::Event;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FocusEvent {
    pub event: Event,
}

impl NativeEvent for FocusEvent {
    fn get_type() -> &'static str where Self: Sized {
        "html.focusevent"
    }
}