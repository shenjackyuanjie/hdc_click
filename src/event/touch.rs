//! docs:
//! https://developer.huawei.com/consumer/cn/doc/harmonyos-guides/uinput#触摸事件
//!

use crate::event::{EventTrait, Pos};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TouchInput {
    /// 按下
    Down(Pos),
    /// 抬起
    Up(Pos),
    /// 点击
    Click {
        pos: Pos,
        /// 点击间隔时间 (按住时间)
        /// 默认 100 [1, 450]
        keep_time: u32,
    },
    /// 移动 (拖动)
    /// 最多支持三指
    Move {
        finger1: (Pos, Pos),
        finger2: Option<(Pos, Pos)>,
        finger3: Option<(Pos, Pos)>,
        /// 按下保持时间
        /// 默认 0 [0, 60000]
        keep_time: u32,
        /// 移动时间
        /// 默认 1000 [1, 15000]
        cost_time: u32,
    },
    /// 单指关节双击
    Knuckle {
        pos1: Pos,
        pos2: Pos,
        /// 间隔时间
        interval_time: u32,
    },
    /// 双指关节双击
    DoubleKnuckle {
        pos1: Pos,
        pos2: Pos,
        /// 间隔时间
        /// 默认: 200 [1, 250]
        interval_time: u32,
    },
}

impl EventTrait for TouchInput {
    // fn type_name(&self) -> &'static str {
    //     match self {
    //         TouchInput::Down(_) => "down",
    //         TouchInput::Up(_) => "up",
    //         TouchInput::Click { .. } => "click",
    //         TouchInput::Move { .. } => "move",
    //         TouchInput::Knuckle { .. } => "knuckle",
    //         TouchInput::DoubleKnuckle { .. } => "double_knuckle",
    //     }
    // }

    fn is_extendable(&self, other: &Self) -> bool {
        use TouchInput::*;
        matches!(
            (self, other),
            (Down(_), Down(_)) | (Down(_), Up(_)) | (Up(_), Down(_))
        )
    }
}
