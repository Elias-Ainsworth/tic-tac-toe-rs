/// Import app as a public module.
pub mod app;
/// Import clap as a public module.
pub mod clap;
/// Import cli as a public module.
pub mod cli;
/// Import completions as a public module.
pub mod completions;
/// Import logic as a public module.
pub mod logic;

/// Publically use functions and enums from app.
pub use app::*;
/// Publically use functions and enums from clap.
pub use clap::*;
/// Publically use the run_game function from the run_game module from the cli module.
pub use cli::run_game::run_game;
/// Publically use the completions function from the generate module.
pub use completions::completions;
/// Publically use the GameState struct, the Generate trait for GameState, and the GameStatus, and Player enums from the logic module.
pub use logic::{GameState, GameStatus, Generate, Player};
