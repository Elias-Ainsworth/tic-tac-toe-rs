#![allow(non_snake_case)]

use crate::{
    logic::{GameState, GameStatus, Generate, Player},
    BoardArgs,
};

use dioxus::prelude::*;

// TODO: Add support for save/load game state as well as undo/redo
// TODO: Add styling
// TODO: Add support for user-defined board sizes
// TODO: Detailed comments and documentation like for the cli

pub fn App() -> Element {
    // Initialize game state
    let board_args = BoardArgs {
        size: Some(3), // Default to 3x3
    };

    // Signals for state management
    let mut game_state: Signal<GameState> =
        use_signal(|| GameState::new(&board_args).expect("Failed to initialize game state"));
    let mut current_player = use_signal(|| Player::X);
    let mut game_status_message = use_signal(|| String::from("Game in progress...")); // Feedback signal

    let board_size = board_args.size.unwrap_or(3); // Default to 3x3 if size is None

    let state = game_state.read();
    let board = state.board.clone();

    // Render the app
    rsx! {
        div {
            h1 { "Tic-Tac-Toe" }

            // Render the game board
            div { style: "display: grid; grid-template-columns: repeat({board_size}, 50px);",
                {
                        board.iter().enumerate().map(|(index, cell)| {
                            rsx! {
                                button {
                                    style: "width: 50px; height: 50px; text-align: center; margin: 1; padding: 1; border: 1px solid black;",
                                    onclick: move |_| {
                                        let mut state = game_state.write(); // Mutably borrow game state

                                        // If cell is empty, update it
                                        if state.board[index] == ' ' {
                                            state.board[index] = current_player.read().to_string().chars().next().unwrap();

                                            // Check game status
                                            match GameState::check_winner(&board_args, (*state).clone()) {
                                                Ok(GameStatus::Won(winner)) => {
                                                    game_status_message.set(format!("Player {} wins!", winner));
                                                },
                                                Ok(GameStatus::Draw) => {
                                                    game_status_message.set(String::from("It's a draw!"));
                                                },
                                                Ok(GameStatus::Ongoing) => {
                                                    // Switch player
                                                    let next_player = match *current_player.read() {
                                                        Player::X => Player::O,
                                                        Player::O => Player::X,
                                                    };
                                                    current_player.set(next_player);
                                                },
                                                Err(e) => {
                                                    game_status_message.set(format!("Error: {}", e));
                                                },
                                            }
                                        }
                                    },
                                    "{cell}",
                                },
                            }
                        })     }
            }

            // Display current player
            div { "Current Player: {current_player.read()}" }

            // Display game status or feedback
            div { "Game Status: {game_status_message.read()}" }
        }
    }
}
