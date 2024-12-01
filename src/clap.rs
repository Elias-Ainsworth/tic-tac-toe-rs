use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Subcommand, ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum ShellCompletion {
    Bash,
    Zsh,
    Fish,
}

#[derive(Args, Debug, PartialEq, Eq)]
pub struct GenerateArgs {
    #[arg(value_enum, help = "Type of Shell completion to generate")]
    pub shell: ShellCompletion,
}

#[derive(Args, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardArgs {
    #[arg(name = "NUMBER", help = "Size of board.", default_value = "3")]
    pub size: Option<usize>,
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum TicTacToeSubcommand {
    #[command(name = "generate", about = "Generate shell completions", hide = true)]
    Generate(GenerateArgs),
    #[command(name = "board_size", about = "Size of board.")]
    Board(BoardArgs),
}

/// Main entry for running the game.
#[derive(Parser, Debug)]
#[command(
    name = "tic-tac-toe-rs",
    about = "A semi-simple tic-tac-toe app written in rust"
)]
pub struct TicTacToeArgs {
    #[command(subcommand)]
    pub command: Option<TicTacToeSubcommand>,

    // Option to launch gui.
    #[arg(long, short = 'l', default_value = "false")]
    pub launch_app: Option<bool>,
}
