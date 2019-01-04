pub struct Game {
    config: Config
}


impl Game {
    pub fn start(&mut self) {
        use super::interface;
        interface::ask_and_run_command(&mut self.config);
    }
}

impl Default for Game {
    fn default() -> Game {
        Game{
            config: Default::default(),
        }
    }
}

use super::board::BoardSize;

pub struct Config {
    pub mode: Mode,
    pub players: (Player, Player),
    pub first: Players,
    board_size: BoardSize,
    pub empty_piece: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            mode: Mode::PVC,
            players: (
                Player {
                    name: String::from("User1"),
                    piece: String::from("X"),
                },
                Player {
                    name: String::from("User2"),
                    piece: String::from("O"),
                }),
            first: Players::Player1,
            board_size: BoardSize::Small,
            empty_piece: String::from("."),
        }
    }
}

impl Config {
    pub fn text_print_config(&self, with_number: bool) {
        let prefixs: [&str; 6] = if with_number {  ["  ";6] 
        } else {
             [
                "  [1] ", 
                "  [2] ", 
                "  [3] ", 
                "  [4] ", 
                "  [5] ", 
                "  [6] "]
        };

        println!("{}Mode: {}", prefixs[0], self.mode);
        println!("{}Player1 ({}): \"{}\"", prefixs[1], self.players.0.name, self.players.0.piece);
        println!("{}Player2 ({}): \"{}\"", prefixs[2], self.players.1.name, self.players.1.piece);
        println!("{}Whose turn first: {}", prefixs[3], self.first);
        println!("{}Board size: {}", prefixs[4], self.board_size);
        println!("{}Empty piece: \"{}\"", prefixs[5], self.empty_piece);
    }

    pub fn text_print_player_page_config(&self) {
        println!("  [1] Player1 name: {}", self.players.0.name);
        println!("  [2] Player2 name: {}", self.players.1.name);
        println!("  [3] Player1 piece: {}", self.players.0.piece);
        println!("  [4] Player2 piece: {}", self.players.1.piece);
    }
}

pub enum Mode {
    PVP,
    PVC,
    CVC,
}


pub struct Player {
    pub name: String,
    pub piece: String,
}

pub enum Players {
    Player1,
    Player2,
}


use std::fmt;
impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match &self {
                Mode::PVP => "PVP",
                Mode::PVC => "PVC",
                Mode::CVC => "CVC",
            }
        )
    }
}

impl fmt::Display for Players {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match &self {
                Players::Player1 => "Player1",
                Players::Player2 => "Player2",
            }
        )
    }
}

