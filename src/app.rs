#![allow(non_snake_case)]

use crate::logic::{GameStatus, Player};
use dioxus::prelude::*;

pub fn App() -> Element {
    let mut current_player = Player::X;
    let mut board_size = 3;
    let mut game_status = GameStatus::Ongoing;

    let on_click = move |_| {
        let new_game = match game_status {
            GameStatus::Ongoing => true,      // TODO: implement game logic
            GameStatus::Won(player) => false, // TODO: implement game logic
            GameStatus::Draw => false,        // TODO: implement game logic
        };

        if new_game {
            current_player = current_player.other();
            board_size = board_size + 1;
            game_status = GameStatus::Ongoing;
        }
    };
    rsx! {
        div {
            h1 {"Tic-Tac-Toe"}
            div {
                button {
                    "New Game"
                },
            }
        }
    }
}
