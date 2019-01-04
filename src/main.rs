#[macro_use]
extern crate clap;
use clap::App;

use tictactoe_rust::game::Game;

fn main() {
    let _m = App::new("tictactoe-rust")
        .version(crate_version!())
        .author("Alex Lai <alexlai97@pm.me>")
        .get_matches();

    let mut game: Game = Default::default();
    game.start();
}
