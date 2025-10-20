// ===== SYSTEM MODULES =====

pub mod environment;
pub mod interaction;
pub mod lifecycle;
pub mod movement;
pub mod setup;
pub mod stats;
pub mod ui;

// Re-export systems for easy access
pub use environment::*;
pub use interaction::*;
pub use lifecycle::*;
pub use movement::*;
pub use setup::*;
pub use stats::*;
pub use ui::*;
