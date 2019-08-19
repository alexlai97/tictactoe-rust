#[macro_use]
extern crate clap;
use clap::{App, Arg};

use tictactoe_rust::game::{Game, Mode};


fn main() {
    let matches = App::new("tictactoe-rust")
        .version(crate_version!())
        .author("Alex Lai <alexlai97@pm.me>")
        .arg(
            Arg::with_name("mode")
                .short("m")
                .long("mode")
                .takes_value(true)
                .help(
                    "Quick start with mode Player vs Player (PVP)
Player vs Computer (PVC)
Computer vs Computer (CVC)",
                )
                .possible_values(&["PVP", "PVC", "CVC"]),
        )
        .get_matches();

    let mode = match matches.value_of("mode").unwrap_or("PVC") {
        "PVP" => Mode::PVP,
        "PVC" => Mode::PVC,
        "CVC" => Mode::CVC,
        _ => Mode::PVC,
    };

    let mut game: Game = Game::new(mode);
    game.start();
}
