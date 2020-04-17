use ragnar_lib::NativeEvent;
use crate::event::Event;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Button {
    Main,
    Auxiliary,
    Secondary,
    Fourth,
    Fifth,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PixelPosition(usize);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MouseEvent {
    pub alt_key: bool,
    pub ctrl_key: bool,
    pub meta_key: bool,
    pub shift_key: bool,
    pub button: Button,
    pub client_x: PixelPosition,
    pub client_y: PixelPosition,
    pub movement_x: PixelPosition,
    pub movement_y: PixelPosition,
    pub page_x: PixelPosition,
    pub page_y: PixelPosition,
    pub screen_x: PixelPosition,
    pub screen_y: PixelPosition,
    pub event: Event,
}

impl NativeEvent for MouseEvent {}