use serde::{Deserialize, Serialize};

use super::Player;

/// Holds the status of the current game and if won who won it.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum GameStatus {
    Ongoing,
    Won(Player),
    Draw,
}
