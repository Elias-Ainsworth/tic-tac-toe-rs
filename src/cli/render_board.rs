use crossterm::{
    cursor::{self},
    style::{Color, Print, SetForegroundColor},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{self};

use crate::GameState;

/// Function to render the game board in the terminal.
pub fn render_board(
    state: &GameState,
    board_size: usize,
    cursor_pos: (usize, usize),
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout: io::Stdout = io::stdout();
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;

    // Render top border.
    render_borders(&mut stdout, board_size, true)?;

    // Render rows and middle borders.
    render_rows(board_size, &mut stdout, state, cursor_pos)?;

    // Render bottom border.
    render_borders(&mut stdout, board_size, false)?;

    // Set cursor positions for cells.
    stdout.execute(cursor::MoveTo(
        (cursor_pos.0 * 4 + 3) as u16,
        (cursor_pos.1 * 2 + 1) as u16,
    ))?;
    Ok(())
}

/// Function to render the borders of the game board.
fn render_borders(
    stdout: &mut io::Stdout,
    board_size: usize,
    is_top: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // If is_top is true
    if is_top {
        // print the top left corner,
        stdout.execute(Print(" ┌"))?;
        // and for each value in the range from 0 to board_size (not inclusive)
        for x in 0..board_size {
            // print a horizontal border
            stdout.execute(Print("───"))?;
            // and immediately after if the value is less than board_size - 1 print a horizontal border with an downward separator
            if x < board_size - 1 {
                stdout.execute(Print("┬"))?;
            }
        }
        // finally print the top right corner and move the cursor to the beginning of the next line.
        stdout.execute(Print("┐"))?;
        stdout.execute(cursor::MoveToNextLine(0))?;
    }
    // If is_top is false
    else {
        // print the bottom left corner,
        stdout.execute(Print(" └"))?;
        // and for each value in the range from 0 to board_size (not inclusive)
        for x in 0..board_size {
            // print a horizontal border
            stdout.execute(Print("───"))?;
            // and immediately after if the value is less than board_size - 1 print a horizontal border with an upward separator
            if x < board_size - 1 {
                stdout.execute(Print("┴"))?;
            }
        }
        // finally print the bottom right corner and move the cursor to the beginning of the next line.
        stdout.execute(Print("┘"))?;
        stdout.execute(cursor::MoveToNextLine(0))?;
    }
    Ok(())
}

/// Function to render the rows of the game board.
fn render_rows(
    board_size: usize,
    stdout: &mut io::Stdout,
    state: &GameState,
    cursor_pos: (usize, usize),
) -> Result<(), Box<dyn std::error::Error>> {
    // For each value y in the range 0..board_size (not inclusive)
    for y in 0..board_size {
        // print a vertical border (prints the separator-less left-hand vertical borders)
        stdout.execute(Print(" │"))?;
        // and for each value x in range 0..board_size (not inclusive) (prints the separator-less middle vertical boarders)
        for x in 0..board_size {
            // let the index be y * board_size + x
            let index = y * board_size + x;
            // and let each cell be the board at the index previously defined.
            let cell = state.board[index];

            // If the the cell at (x, y) is equal to the current cursor position highlight the char blue
            if (x, y) == cursor_pos {
                stdout.execute(SetForegroundColor(Color::Blue))?;
            }
            // and if the cell is not empty highlight the char green
            else if cell != ' ' {
                stdout.execute(SetForegroundColor(Color::Green))?;
            }
            // otherwise highlight it white.
            else {
                stdout.execute(SetForegroundColor(Color::White))?;
            }
            // Print each cell after going through the highlighting.
            stdout.execute(Print(format!(" {} ", cell)))?;
            // Highlight the rest (the borders) white.
            stdout.execute(SetForegroundColor(Color::White))?;

            // If the x value is less than the board_size - 1 print a vertical border after each cell in loop.
            if x < board_size - 1 {
                stdout.execute(Print("│"))?;
            }
        }
        // Print a vertical border after all the x values are iterated (prints the separator-less right-hand vertical borders).
        stdout.execute(Print("│"))?;
        // // Move to the beginning of the next line and immediately after
        stdout.execute(cursor::MoveToNextLine(0))?;
        // if the y value is less than board_size - 1 (to make sure it does not print it at the bottom)
        if y < board_size - 1 {
            // print a vertical border with a right-ward separator
            stdout.execute(Print(" ├"))?;
            // and immediately after for each value x in the range 0..board_size (not inclusive)
            for x in 0..board_size {
                // print a horizontal border
                stdout.execute(Print("───"))?;
                // and if x is less than board_size - 1 print a vertical border with both a left-ward and right-ward separator (to make sure it does not print it at the end)
                if x < board_size - 1 {
                    stdout.execute(Print("┼"))?;
                }
            }
            // and finally print a vertical border with a left-ward separator.
            stdout.execute(Print("┤"))?;
            stdout.execute(cursor::MoveToNextLine(0))?;
        }
    }
    Ok(())
}
