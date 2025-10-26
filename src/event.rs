pub mod keyboard;
pub mod mouse;
pub mod stylus;
pub mod touch;
pub mod touchpad;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

impl Pos {
    pub fn new(x: u32, y: u32) -> Self {
        Pos { x, y }
    }
}

pub fn pos(x: u32, y: u32) -> Pos {
    Pos::new(x, y)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEventType {
    Mouse(mouse::MouseInput),
    KeyBoard(keyboard::KeyboardInput),
    Stylus(stylus::StylusInput),
    Touch(touch::TouchInput),
    TouchPad(touchpad::TouchPadInput),
    /// 等待
    /// 单位: ms
    Sleep(u32),
}

impl InputEventType {
    /// 判断两个事件是否为相同子类型
    pub fn is_same_variant(&self, other: &InputEventType) -> bool {
        use InputEventType::*;

        match (self, other) {
            (Mouse(a), Mouse(b)) => std::mem::discriminant(a) == std::mem::discriminant(b),
            (KeyBoard(a), KeyBoard(b)) => std::mem::discriminant(a) == std::mem::discriminant(b),
            (Stylus(a), Stylus(b)) => std::mem::discriminant(a) == std::mem::discriminant(b),
            (Touch(a), Touch(b)) => std::mem::discriminant(a) == std::mem::discriminant(b),
            (TouchPad(a), TouchPad(b)) => std::mem::discriminant(a) == std::mem::discriminant(b),
            (Sleep(_), Sleep(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub inputs: Vec<InputEventType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UinputCommand {
    Cmd(String),
    Pause(u32),
}

impl UinputCommand {
    pub fn new(cmd: &str) -> Self {
        UinputCommand::Cmd(cmd.to_string())
    }
    pub fn pause(ms: u32) -> Self {
        UinputCommand::Pause(ms)
    }
}

pub trait EventTrait {
    /// 事件名
    // fn type_name(&self) -> &'static str;
    /// 是否可扩展 ( 或者说生成一个合并后的 cmd )
    fn is_extendable(&self, other: &Self) -> bool {
        let _ = other;
        false
    }
}

impl EventTrait for InputEventType {
    // fn type_name(&self) -> &'static str {
    //     match self {
    //         InputEventType::Key(_) => "Key",
    //         InputEventType::Mouse(_) => "Mouse",
    //         InputEventType::Sleep(_) => "Sleep",
    //     }
    // }
    fn is_extendable(&self, other: &Self) -> bool {
        use InputEventType::*;
        if std::mem::discriminant(self) == std::mem::discriminant(other) {
            match (self, other) {
                (KeyBoard(k1), KeyBoard(k2)) => k1.is_extendable(k2),
                (Mouse(m1), Mouse(m2)) => m1.is_extendable(m2),
                (Stylus(s1), Stylus(s2)) => s1.is_extendable(s2),
                (Touch(t1), Touch(t2)) => t1.is_extendable(t2),
                (TouchPad(t1), TouchPad(t2)) => t1.is_extendable(t2),
                (Sleep(_), Sleep(_)) => true,
                _ => false,
            }
        } else {
            false
        }
    }
}

impl Event {
    pub fn empty() -> Self {
        Self { inputs: Vec::new() }
    }
    pub fn and(&mut self, event: InputEventType) -> &mut Self {
        self.inputs.push(event);
        self
    }
    pub fn and_more(&mut self, events: &[InputEventType]) -> &mut Self {
        self.inputs.extend_from_slice(events);
        self
    }
    pub fn sleep(&mut self, ms: u32) -> &mut Self {
        self.inputs.push(InputEventType::Sleep(ms));
        self
    }
    pub fn is_empty(&self) -> bool {
        self.inputs.is_empty()
    }
    pub fn build(&self) -> Vec<UinputCommand> {
        if self.is_empty() {
            return Vec::new();
        }
        let mut cmds = Vec::new();
        let mut this_cmd = &self.inputs[0];
        let mut ptr = 0;
        loop {
            if ptr >= self.inputs.len() {
                break;
            }
            // 尝试合并
            // 前提是有下一个
            if ptr + 1 < self.inputs.len() {
                if std::mem::discriminant(this_cmd) == std::mem::discriminant(&self.inputs[ptr + 1])
                {
                    let this = this_cmd;
                    let next = &self.inputs[ptr + 1];
                } else {
                }
            } else {
                break;
            }
        }
        cmds
    }
}
