#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseInput {

}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseInputType {
    Move(),
    Press(),
    Lift(),
    Click(),
    DoubleClick(),
    Scroll(),
    Drag(),
}
