// Set module render_board as public so that run_game() can use it.
pub mod render_board;
// Set module run_game as public so that main() can use it.
pub mod run_game;
// Set the save file as a public constant to avoid redundancy.
pub const SAVE_FILE: &str = "save_game.json";
