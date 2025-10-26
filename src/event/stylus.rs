//! docs:
//! https://developer.huawei.com/consumer/cn/doc/harmonyos-guides/uinput#触控笔事件
//!

use crate::event::EventTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StylusInput {
    Down {
        x: u32,
        y: u32,
    },
    Up {
        x: u32,
        y: u32,
    },
    Move {
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
        smooth_time: u32,
        keep_time: u32,
    },
    Click {
        x: u32,
        y: u32,
        keep_time: u32,
    },
    Drag {
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
        press_time: u32,
        drag_time: u32,
    },
}

impl EventTrait for StylusInput {}
