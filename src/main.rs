use clap::Parser;
// use dioxus::{desktop::Config, prelude::LaunchBuilder};
use tic_tac_toe_rs::{
    completions, run_game, /* App,*/ BoardArgs, TicTacToeArgs, TicTacToeSubcommand,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: TicTacToeArgs = TicTacToeArgs::parse();
    if let Some(app) = args.launch_app {
        match app {
            /* true => {
                LaunchBuilder::desktop()
                    .with_cfg(
                        Config::new()
                            .with_background_color((30, 30, 46, 255))
                            .with_menu(None)
                            // disable on release builds
                            .with_disable_context_menu(!cfg!(debug_assertions))
                            .with_custom_index(
                                r#"<!DOCTYPE html>
                                    <html>
                                        <head>
                                            <title>Dioxus app</title>
                                            <meta name="viewport" content="width=device-width, initial-scale=1.0">
                                            <link rel="stylesheet" href="public/tailwind.css">
                                        </head>
                                        <body>
                                            <div id="main" style="height: 100vh;"></div>
                                        </body>
                                    </html>"#.to_string(),
                        ),
                    )
                    .launch(App);
                Ok(())
            } */
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
