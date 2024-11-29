use crate::BoardArgs;
/// Enum to represent player in the game: either X or O.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

/// Holds the state of the game, including the board.
#[derive(Clone, Debug)]
pub struct GameState {
    pub board: Vec<char>,
    pub current_player: Player,
    pub game_over: bool,
}

/// Errors that can occur during game operations.
#[derive(Debug)]
enum GameError {
    InvalidBoardSize,
    EmptyBoard,
}

/// Implementation to display message based on error.
impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::InvalidBoardSize => write!(f, "The board size is invalid."),
            GameError::EmptyBoard => write!(f, "The board is empty."),
        }
    }
}

impl std::error::Error for GameError {}

/// Trait for generating and checking the game board.
pub trait Generate {
    /// Creates a new empty board based on the provided arguments.
    fn new(boardargs: &BoardArgs) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized; // Make sure size of Self is set at compile time.
    /// Checks if there is a winner on the board.
    fn check_winner(
        boardargs: &BoardArgs,
        state: GameState,
    ) -> Result<Option<char>, Box<dyn std::error::Error>>;
}

impl Generate for GameState {
    fn new(boardargs: &BoardArgs) -> Result<Self, Box<dyn std::error::Error>> {
        // Set size of board and handle errors.
        let size: usize = match boardargs.size {
            Some(size) => size,
            None => Err(GameError::InvalidBoardSize)?,
        };
        // Initialize the board as a vector of spaces, with the number of elements equal to size^2.
        let board: Vec<char> = vec![' '; size * size];
        // Return board as an Ok value.
        Ok(GameState {
            board,
            current_player: Player::X, // Default starting player.
            game_over: false,          // Default game status.
        })
    }
    fn check_winner(
        boardargs: &BoardArgs,
        state: Self,
    ) -> Result<Option<char>, Box<dyn std::error::Error>> {
        // Get size of board.
        let size: usize = boardargs.size.unwrap();
        // Check if the board is valid size.
        if state.board.len() != size * size {
            return Err(Box::new(GameError::InvalidBoardSize));
        }
        // Check if the board is empty.
        if state.board.is_empty() {
            return Err(Box::new(GameError::EmptyBoard));
        }
        // If there are no errors get board.
        let board = &state.board;
        // Get winning combinations.
        let winning_combinations: Vec<Vec<usize>> = (0..size) // Sets the range as from 0 to size - 1
            .flat_map(|index: usize| {
                // For each index (row and column), create a row and column vector:

                // Creates a vector containing the values of a row.
                // For example if the index = 0, and size = 3 the row's values would be: 0..3 = [0, 1 ,2] and repeats for each value in index.
                let row: Vec<usize> = (index * size..(index + 1) * size).collect::<Vec<usize>>();

                // Creates a vector containing the values of a column.
                // For example if the index = 0, and size = 3 the column's values would be: 0..9 = [0, 1, 2, 3, 4, 5, 6, 7, 8].
                // This would obviously be problematic that's why the step_by() takes the initial index and every size-th element meaning it returns [0, 3, 6] and repeats for each value in index.
                let col: Vec<usize> = (index..size * size).step_by(size).collect::<Vec<usize>>();

                // Combines the row and column vectors into a single vector and returns them.
                vec![row, col]
            })
            // The chain method concatenates the results of the rows and columns with the horizontals withing the chain method.
            .chain(vec![
                // Creates diagonal from top-left to bottom-right.
                // The step_by(size + 1) returns the next diagonal value from the flattened range.
                // For example if the index = 0, and size = 3 the diagonal's value would be: 0..9, and the step_by() method simply returns the initial index and every size + 1-th element.
                // Which would then in-turn return: [2, 4, 6].
                (0..size * size).step_by(size + 1).collect(),
                // Creates diagonal from top-right to bottom-left.
                // The step_by(size - 1) returns the next diagonal value from the flattened range.
                // For example if the index = 0, and size = 3 the diagonal's value would be: 2..8, and the step_by() method simply returns the initial index and every size - 1-th element.
                // Which would then in-turn return: [2, 4, 6].
                ((size - 1)..(size * size - 1)).step_by(size - 1).collect(),
            ])
            // Collects the chained iterater into a vector of vectors containing values of usize which holds the winning combinations.
            .collect::<Vec<Vec<usize>>>();
        // Go through each winning combination to see if given board matches any.
        for combo in winning_combinations {
            // Gets the value of the player at index 0
            let first: char = board[combo[0]];
            if first != ' ' // If the first char is not empty
                && combo // and each value in the winning combinaitons are the first char
                    .iter()
                    .all(|&position_index: &usize| board[position_index] == first)
            {
                return Ok(Some(first)); // return the value of the first index as the winner.
            }
        }
        Ok(None)
    }
}
