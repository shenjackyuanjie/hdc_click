pub mod mouse;
pub mod keyboard;
pub mod stylus;
pub mod touch;
pub mod touchpad;

pub enum InputEventType {
    Mouse(),
    KeyBoard(),
    Stylus(),
    Touch(),
    TouchPad(),
}
