use crate::{BoardArgs, GameState, Generate, Player};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};

/// Function to render the game board in the terminal.
fn render_board(
    state: &GameState,
    size: usize,
    cursor_pos: (usize, usize),
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout: io::Stdout = io::stdout();
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;

    for y in 0..size {
        for x in 0..size {
            let index = y * size + x;
            let cell = state.board[index];

            if (x, y) == cursor_pos {
                stdout.execute(SetForegroundColor(Color::Yellow))?;
            } else {
                stdout.execute(SetForegroundColor(Color::White))?;
            }

            stdout.execute(Print(format!(" {} ", cell)))?;

            if x < size - 1 {
                stdout.execute(Print("|"))?;
            }
        }
        if y < size - 1 {
            stdout.execute(Print("\n"))?;
            stdout.execute(Print("-".repeat(size * 4 - 1)))?;
            stdout.execute(Print("\n"))?;
        }
    }
    stdout.execute(cursor::MoveTo(
        (cursor_pos.1 * 4) as u16,
        (cursor_pos.0 * 2) as u16,
    ))?;
    Ok(())
}

pub fn run_game(boardargs: &BoardArgs) -> Result<(), Box<dyn std::error::Error>> {
    let size: usize = boardargs.size.unwrap_or(3);
    let mut state: GameState = GameState {
        board: vec![' '; size * size],
        current_player: Player::X,
        game_over: false,
    };

    let mut cursor_pos: (usize, usize) = (0, 0);

    terminal::enable_raw_mode()?;

    loop {
        render_board(&state, size, cursor_pos)?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up => {
                    println!("Up Arrow pressed");
                    if cursor_pos.1 > 0 {
                        cursor_pos.1 -= 1;
                    }
                }
                KeyCode::Down => {
                    println!("Up Down pressed");
                    if cursor_pos.1 < size - 1 {
                        cursor_pos.1 += 1;
                    }
                }
                KeyCode::Left => {
                    println!("Up Left pressed");
                    if cursor_pos.0 > 0 {
                        cursor_pos.0 -= 1;
                    }
                }
                KeyCode::Right => {
                    println!("Up Right pressed");
                    if cursor_pos.0 < size - 1 {
                        cursor_pos.0 += 1;
                    }
                }
                KeyCode::Enter => {
                    println!("Up Enter pressed");
                    let idx = cursor_pos.1 * size + cursor_pos.0;
                    if state.board[idx] == ' ' {
                        state.board[idx] = state.current_player.to_string().chars().next().unwrap();
                        if let Ok(Some(winner)) = GameState::check_winner(boardargs, state.clone())
                        {
                            state.game_over = true;
                            println!("Player {} wins!", winner);
                            break;
                        }
                        state.current_player = match state.current_player {
                            Player::X => Player::O,
                            Player::O => Player::X,
                        };
                    }
                }
                KeyCode::Esc => {
                    println!("Up Esc pressed");
                    break;
                }
                _ => {}
            }
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}
