use serde::{Deserialize, Serialize};

/// Enum to represent player in the game: either X or O.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

/// Implementation to display player as a string ("X" or "O").
impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}
