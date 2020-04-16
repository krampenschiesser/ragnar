use ragnar_lib::NativeEvent;

#[derive(Debug, Clone)]
pub enum Button {
    Main,
    Auxiliary,
    Secondary,
    Fourth,
    Fifth,
}

#[derive(Debug, Clone)]
pub struct PixelPosition(usize);

#[derive(Debug, Clone)]
pub struct MouseEvent {
    alt_key: bool,
    ctrl_key: bool,
    meta_key: bool,
    shift_key: bool,
    button: Button,
    client_x: PixelPosition,
    client_y: PixelPosition,
    movement_x: PixelPosition,
    movement_y: PixelPosition,
    page_x: PixelPosition,
    page_y: PixelPosition,
    screen_x: PixelPosition,
    screen_y: PixelPosition,
}

impl NativeEvent for MouseEvent {}