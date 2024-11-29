use clap::Parser;
use tic_tac_toe_rs::{completions, run_game, BoardArgs, TicTacToeArgs, TicTacToeSubcommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: TicTacToeArgs = TicTacToeArgs::parse();
    if let Some(app) = args.launch_app {
        match app {
            _ => match args.command.as_ref() {
                Some(command) => match command {
                    TicTacToeSubcommand::Generate(args) => completions(args),
                    TicTacToeSubcommand::Board(args) => run_game(args),
                },
                None => {
                    let default = run_game(&BoardArgs { size: Some(3) });
                    return default;
                }
            },
        }
    } else {
        Ok(())
    }
}
