use crossterm::{
    cursor::{self, SetCursorStyle},
    event::{self, Event, KeyCode},
    style::{Color, Print, SetBackgroundColor},
    terminal::{self},
    ExecutableCommand,
};
use std::{
    io::{self},
    time::Duration,
};

use crate::cli::render_board::render_board;
use crate::{BoardArgs, GameState, GameStatus, Generate, Player};

use super::SAVE_FILE;

/// Function to run the game in the terminal.
pub fn run_game(boardargs: &BoardArgs) -> Result<(), Box<dyn std::error::Error>> {
    // Enable raw mode to filter inputs and outputs.
    terminal::enable_raw_mode()?;

    let board_size: usize = boardargs.size.unwrap_or(3);
    let mut state: GameState = GameState::new(boardargs)?;

    let mut cursor_pos: (usize, usize) = (0, 0);

    let mut stdout = io::stdout();

    // Prompt user to load game from a file or not.
    load_screen(&mut stdout, &mut state, board_size, cursor_pos)?;

    // Set cursor style to underscore for the sake of sanity.
    stdout.execute(SetCursorStyle::SteadyUnderScore)?;

    // Repeats code until either a win or a draw causes a break.
    loop {
        // render board
        render_board(&state, board_size, cursor_pos)?;
        // Read key events and map each to vary cursor positions.
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    // Subtracts 1 from y value while it is greater than 0.
                    if cursor_pos.1 > 0 {
                        cursor_pos.1 -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    // Adds 1 to y value while it is less than board_size -1.
                    if cursor_pos.1 < board_size - 1 {
                        cursor_pos.1 += 1;
                    }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    // Subtracts 1 from x value while it is greater than 0.
                    if cursor_pos.0 > 0 {
                        cursor_pos.0 -= 1;
                    }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    // Adds 1 to x value while it is less than board_size -1.
                    if cursor_pos.0 < board_size - 1 {
                        cursor_pos.0 += 1;
                    }
                }
                KeyCode::Enter | KeyCode::Char('w') => {
                    // Sets the board_index to the y value times the board_size plus the x value.
                    let board_index = cursor_pos.1 * board_size + cursor_pos.0;
                    // If the char in the board at board_index is empty then set that char to the current player as a char.
                    if state.board[board_index] == ' ' {
                        state.board[board_index] = state
                            .current_player
                            .to_string()
                            .chars()
                            .next()
                            .ok_or("Failed to get player character")?;
                        // Save state to save file (saving the new char).
                        state.save_game(SAVE_FILE)?;

                        // Matches the value of GameState::check_winner() to find the GameStatus and act accordingly
                        match GameState::check_winner(boardargs, state.clone()) {
                            // If the returned value is Ok() then match the value of status.
                            Ok(status) => match status {
                                // If the status returned is Won, then end the game and return the winner.
                                GameStatus::Won(winner) => {
                                    state.status = GameStatus::Won(winner);

                                    render_board(&state, board_size, cursor_pos)?;

                                    // Move to the bottom of the board.
                                    stdout
                                        .execute(cursor::MoveTo(1, (board_size * 2 + 1) as u16))?;
                                    stdout.execute(SetBackgroundColor(Color::Green))?;
                                    stdout.execute(Print(format!("Player {} wins!", winner)))?;
                                    stdout.execute(SetBackgroundColor(Color::Reset))?;
                                    stdout.execute(cursor::MoveToNextLine(0))?;

                                    // Save state to save file (saving the GameStatus).
                                    state.save_game(SAVE_FILE)?;
                                    break;
                                }
                                // If the status returned is Draw, then end the game and report it.
                                GameStatus::Draw => {
                                    state.status = GameStatus::Draw;

                                    render_board(&state, board_size, cursor_pos)?;

                                    // Move to the bottom of the board.
                                    stdout
                                        .execute(cursor::MoveTo(1, (board_size * 2 + 1) as u16))?;
                                    stdout.execute(SetBackgroundColor(Color::DarkGrey))?;
                                    stdout.execute(Print(format!("It's a draw!")))?;
                                    stdout.execute(SetBackgroundColor(Color::Reset))?;
                                    stdout.execute(cursor::MoveToNextLine(0))?;

                                    // Save state to save file (saving the GameStatus).
                                    state.save_game(SAVE_FILE)?;
                                    break;
                                }
                                // If the status returned is Ongoing, then continue the game.
                                GameStatus::Ongoing => {}
                            },
                            // If the value returned is Err(), move to the bottom of the board and report the error.
                            Err(e) => {
                                stdout.execute(cursor::MoveTo(1, (board_size * 2 + 1) as u16))?;
                                stdout.execute(SetBackgroundColor(Color::Red))?;
                                stdout.execute(Print(format!("Error: {}", e)))?;
                                stdout.execute(SetBackgroundColor(Color::Reset))?;
                                stdout.execute(cursor::MoveToNextLine(0))?;
                                break;
                            }
                        }
                        // Go to the next player by using match to swap the values.
                        state.current_player = match state.current_player {
                            Player::X => Player::O,
                            Player::O => Player::X,
                        };

                        // Save state to save file (saving the current_player).
                        state.save_game(SAVE_FILE)?;
                    }
                }
                // If esc is hit break out of the loop and exit the game.
                KeyCode::Esc | KeyCode::Char('q') => {
                    break;
                }
                // If any other key is hit do nothing :3.
                _ => {}
            }
        }
    }
    // Once the loop is broken disable raw mode and return the terminal to its original state.
    terminal::disable_raw_mode()?;
    Ok(())
}

/// Function that prompts the user as to whether or not to load a game from the save file.
fn load_screen(
    stdout: &mut io::Stdout,
    state: &mut GameState,
    board_size: usize,
    cursor_pos: (usize, usize),
) -> Result<(), Box<dyn std::error::Error>> {
    // Repeat so that if an invalid char is inputted the user can try again.
    loop {
        stdout.execute(Print(
            "Do you want to load the previously saved game? (y/n)",
        ))?;
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                // If key 'y' is pressed and result type is Ok load the game from save file if not and result is Err(e) return e.
                KeyCode::Char('y') => match GameState::load_game(SAVE_FILE) {
                    Ok(loaded_state) => {
                        *state = loaded_state;
                        render_board(&*state, board_size, cursor_pos)?;
                        stdout.execute(cursor::MoveTo(0, (board_size * 2 + 1) as u16))?;
                        stdout.execute(SetBackgroundColor(Color::Green))?;
                        stdout.execute(Print("Game loaded successfully."))?;
                        stdout.execute(SetBackgroundColor(Color::Reset))?;
                        // Wait for a second.
                        std::thread::sleep(Duration::from_secs(1));
                        break;
                    }
                    Err(e) => {
                        stdout.execute(cursor::MoveToNextLine(0))?;
                        stdout.execute(SetBackgroundColor(Color::Red))?;
                        stdout.execute(Print(format!("Error: {}", e)))?;
                        stdout.execute(SetBackgroundColor(Color::Reset))?;
                        // Wait for 2 seconds.
                        std::thread::sleep(Duration::from_secs(2));
                        stdout.execute(cursor::MoveToNextLine(0))?;
                        stdout.execute(Print("Loading new empty game."))?;
                        // Wait for 2 seconds.
                        std::thread::sleep(Duration::from_secs(2));
                        break;
                    }
                },
                // If key 'n' is pressed load new game and in the process rewrite save file.
                KeyCode::Char('n') => break,
                _ => {
                    stdout.execute(cursor::MoveToNextLine(0))?;
                }
            }
        }
    }
    Ok(())
}
