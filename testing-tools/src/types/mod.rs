mod events;
use std::sync::OnceLock;

pub use events::*;
mod handles;
pub use handles::*;
mod hooks;
pub use hooks::*;
