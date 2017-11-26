//! Provides OpenGL-based windows, and abstractions for events and input handling.

pub use self::context::Context;
pub use self::context_settings::ContextSettings;
pub use self::event::Event;
pub use self::keyboard::{set_virtual_keyboard_visible, Key};
pub use self::style::Style;
pub use self::video_mode::VideoMode;
pub use self::window::Window;

#[cfg_attr(feature = "cargo-clippy", allow(module_inception))]
mod window;
mod video_mode;
mod context;
mod context_settings;
pub mod joystick;
mod keyboard;
pub mod mouse;
mod event;
mod style;
pub mod sensor;
pub mod touch;
