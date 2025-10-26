use crate::event::EventTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseInput {
    /// 移动
    Move(),
    /// 按下
    Press(),
    /// 抬起
    Lift(),
    /// 点击
    Click(),
    /// 双击
    DoubleClick(),
    /// 滚动
    Scroll(),
    /// 拖动
    Drag(),
}

impl EventTrait for MouseInput {}
