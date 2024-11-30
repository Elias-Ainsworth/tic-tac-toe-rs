use serde::{Deserialize, Serialize};

use super::{game_error::GameError, player::Player, GameStatus};

/// Holds the state of the game; including the board, current player, and game status.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameState {
    pub board: Vec<char>,
    pub current_player: Player,
    pub status: GameStatus,
}

/// Implementation for GameState containing functions to serialize the current game and load game from deserialized save file.
impl GameState {
    /// Save game as serialized json file.
    pub fn save_game(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&self)?;
        std::fs::write(filename, json)?;
        Ok(())
    }
    /// Load game by setting the contents of GameState as the deserialized contents of the json file.
    pub fn load_game(filename: &str) -> Result<GameState, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filename)?;
        let state: GameState = serde_json::from_str(&json)?;
        match state.status {
            GameStatus::Ongoing => Ok(state),
            _ => Err(GameError::GameOver.into()),
        }
    }
}
