use crate::models::WindowHandle;

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    ResizingWindow(WindowHandle),
    MovingWindow(WindowHandle),
    NormalMode,
}
