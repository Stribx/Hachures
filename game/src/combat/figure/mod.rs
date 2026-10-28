//! Enemy figurines: the kit of parts, the look built from it and the spawn that stands one on the
//! board.

pub mod chain;
pub mod exit;
mod gait;
mod jolt;
pub mod kit;
mod look;
mod props;
mod spawn;

pub use chain::drag_chain;
pub use exit::{ExitKind, exits, leave};
pub use gait::{Stride, animate};
pub use look::{EnemyLook, setup};
pub use spawn::{FLOAT, spawn_figure};
