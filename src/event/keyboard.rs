//! docs:
//! https://developer.huawei.com/consumer/cn/doc/harmonyos-guides/uinput#键盘事件
//!

use crate::event::EventTrait;

/// 键盘事件
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyboardInput {
    Down { key: u32 },
    Up { key: u32 },
    Press { key: u32, press_time: u32 },
    Repeat { key: u32, repeat_time: u32 },
    Text { text: Vec<char> },
}

impl EventTrait for KeyboardInput {}
