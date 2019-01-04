use super::board::Board;
pub struct Game {
    config: Config,
    board: Board,
}

impl Game {
    pub fn start(&mut self) {
        use super::interface;
        interface::ask_and_run_command(&mut self.config, &mut self.board);
    }

    pub fn text_print_board(board: &Board, config: &Config) {
        let spaces = " ".repeat(2);
        for (i, player) in board.board.iter().enumerate() {
            let cell: &str = match player {
                Some(Players::Player1) => config.players.0.piece.as_str(),
                Some(Players::Player2) => config.players.1.piece.as_str(),
                None => config.empty_piece.as_str(),
            };

            if (i + 1) % 3 == 0 {
                println!("{}", cell);
            } else if i % 3 == 0 {
                print!("{}{}|", spaces, cell);
            } else {
                print!("{}|", cell);
            }
        }
    }

    pub fn new(mode: Mode) -> Game {
        Game {
            config: Config::new(mode),
            board: Default::default(),
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

impl Config {
    pub fn new(mode: Mode) -> Config {
        match mode {
            Mode::PVC => Config {
                mode: Mode::PVC,
                players: (
                    Player {
                        name: String::from("User1"),
                        piece: String::from("X"),
                        is_ai: false,
                    },
                    Player {
                        name: String::from("AI1"),
                        piece: String::from("O"),
                        is_ai: true,
                    },
                ),
                first: Players::Player1,
                board_size: BoardSize::Small,
                empty_piece: String::from("."),
            },
            Mode::PVP => Config {
                mode: Mode::PVP,
                players: (
                    Player {
                        name: String::from("User1"),
                        piece: String::from("X"),
                        is_ai: false,
                    },
                    Player {
                        name: String::from("User2"),
                        piece: String::from("O"),
                        is_ai: false,
                    },
                ),
                first: Players::Player1,
                board_size: BoardSize::Small,
                empty_piece: String::from("."),
            },
            Mode::CVC => Config {
                mode: Mode::CVC,
                players: (
                    Player {
                        name: String::from("AI1"),
                        piece: String::from("X"),
                        is_ai: true,
                    },
                    Player {
                        name: String::from("AI2"),
                        piece: String::from("O"),
                        is_ai: true,
                    },
                ),
                first: Players::Player1,
                board_size: BoardSize::Small,
                empty_piece: String::from("."),
            },
        }
    }
}

impl Config {
    pub fn text_print_config(&self, with_number: bool) {
        let prefixs: [&str; 6] = if with_number {
            ["  "; 6]
        } else {
            ["  [1] ", "  [2] ", "  [3] ", "  [4] ", "  [5] ", "  [6] "]
        };

        println!("{}Mode: {}", prefixs[0], self.mode);
        println!(
            "{}Player1 ({}): \"{}\"",
            prefixs[1], self.players.0.name, self.players.0.piece
        );
        println!(
            "{}Player2 ({}): \"{}\"",
            prefixs[2], self.players.1.name, self.players.1.piece
        );
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

#[derive(Clone, Copy)]
pub enum Mode {
    PVP,
    PVC,
    CVC,
}

pub struct Player {
    pub name: String,
    pub piece: String,
    pub is_ai: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Players {
    Player1,
    Player2,
}

use std::fmt;
impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
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
        write!(
            f,
            "{}",
            match &self {
                Players::Player1 => "Player1",
                Players::Player2 => "Player2",
            }
        )
    }
}

#[cfg(test)]
mod tests;
