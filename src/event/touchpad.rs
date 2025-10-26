//! docs:
//! https://developer.huawei.com/consumer/cn/doc/harmonyos-guides/uinput#触控板事件
//!

use crate::event::EventTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchPadInput {
    /// 捏合
    Pinch(),
    /// 滑动
    Swipe(),
    /// 旋转
    /// Note: 目前无作用
    /// https://developer.huawei.com/consumer/cn/doc/harmonyos-guides/uinput#触控板旋转事件
    Rotate(),
}

impl EventTrait for TouchPadInput {}
