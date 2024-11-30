/// Errors that can occur during game operations.
#[derive(Debug)]
pub enum GameError {
    InvalidBoardSize,
    EmptyBoard,
    GameOver,
}

/// Implementation of std::fmt::Display for GameError to display message based on error.
impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::InvalidBoardSize => write!(f, "The board size is invalid."),
            GameError::EmptyBoard => write!(f, "The board is empty."),
            GameError::GameOver => write!(f, "Cannot load a game that is over."),
        }
    }
}

/// Implementation of std::error::Error for GameError so that GameError is seen as an error type.
impl std::error::Error for GameError {}
