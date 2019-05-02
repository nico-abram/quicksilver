use crate::{
    input::{ButtonState, GamepadAxis, GamepadButton, Key, MouseButton},
    geom::Vector
};

/// An input event
#[derive(Copy, Clone, Debug)]
pub enum Event {
    /// The application has been closed
    Closed,
    /// The application has gained focus
    Focused,
    /// The application has lost focus
    Unfocused,
    /// A key has changed its button state
    Key(Key, ButtonState),
    /// An alphanumeric character has been entered from the keyboard
    Typed(char),
    /// The mouse has been moved to a position
    MouseMoved(Vector),
    /// The mouse has entered the window
    MouseEntered,
    /// The mouse has exited the window
    MouseExited,
    /// The mouse wheel has been scrolled by a vector
    MouseWheel(Vector),
    /// A mouse button has changed its button state
    MouseButton(MouseButton, ButtonState),
    /// A gamepad axis has changed its state
    GamepadAxis(i32, GamepadAxis, f32),
    /// A gamepad button has changed its state
    GamepadButton(i32, GamepadButton, ButtonState),
    /// A gamepad has been connected
    GamepadConnected(i32),
    /// A gamepad has been disconnected
    GamepadDisconnected(i32)
}

