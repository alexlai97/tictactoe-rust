use ansi_term::Color::*;
static TICTACTOE_LOGO: &str = "
| |_(_) ___| |_ __ _  ___| |_ ___   ___       _ __ _   _ ___| |_ 
| __| |/ __| __/ _` |/ __| __/ _ \\ / _ \\_____| '__| | | / __| __|
| |_| | (__| || (_| | (__| || (_) |  __/_____| |  | |_| \\__ \\ |_ 
 \\__|_|\\___|\\__\\__,_|\\___|\\__\\___/ \\___|     |_|   \\__,_|___/\\__|
";

static GAME_RULE: &str = "Game rule:

Two player put their piece turn by turn.
If a player have 3 same pieces on a line 
horizontally, or vertically, or diagonally,
that player wins.";

static FRONT_PAGE_AVAILABLE_COMMANDS: [Commands; 9] = [
    Commands::Help,
    Commands::Start,
    Commands::Quit,
    Commands::Command1,
    Commands::Command2,
    Commands::Command3,
    Commands::Command4,
    Commands::Command5,
    Commands::Command6,
];
static FRONT_PAGE_HELP_AVAILABLE_COMMANDS: [Commands; 3] =
    [Commands::Start, Commands::Back, Commands::Quit];
static CONFG_PLAYER_AVAILABLE_COMMANDS: [Commands; 6] = [
    Commands::Command1,
    Commands::Command2,
    Commands::Command3,
    Commands::Command4,
    Commands::Back,
    Commands::Quit,
];

static EMPTY_PIECE_TABLE: &'static [(&str, &str, &str)] =
    &[(&"sp", &" ", &"(space)"), (&"dt", &".", &"(dot)")];

static PIECE_TABLE: &'static [(&str, &str, &str)] = &[
    (&"bc", &"●", &"(black circle)"),
    (&"wc", &"○", &"(white circle)"),
    (&"bs", &"■", &"(black square)"),
    (&"ws", &"□", &"(white square)"),
    (&"bk", &"♚", &"(black king)"),
    (&"wk", &"♔", &"(white king)"),
    (&"bq", &"♛", &"(black queen)"),
    (&"wq", &"♕", &"(white queen)"),
    (&"bp", &"♟", &"(black pawn)"),
    (&"wp", &"♙", &"(white pawn)"),
];

use super::board::Board;
use super::game::Config;

fn print_table(table: &[(&str, &str, &str)], num_of_spaces: usize) {
    let spaces = " ".repeat(num_of_spaces);
    for i in table {
        println!("{}[{}] {} {}", spaces, i.0, i.1, i.2);
    }
}

fn choose_string_from_table(
    table: &'static [(&str, &'static str, &str)],
    string: &str,
) -> Option<&'static str> {
    for i in table {
        if string == i.0 {
            return Some(i.1);
        }
        continue;
    }
    None
}

fn ask_for_user_name() -> String {
    use std::io;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line when asking for user command");
    let input = input.trim();
    input.to_string()
}

fn ask_user_move_input() -> Result<(usize, usize), &'static str> {
    use std::io;
    println!("Where you want to put your piece: Ex: 0, 2 or (q)uit");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line when asking for user input");

    let input = input.trim();

    if input == "q" || input == "quit" {
        return Err("Quit");
    }

    let string_pool: Vec<&str> = input.split(|c| c == ' ' || c == ',').collect();

    let nums: Vec<usize> = string_pool
        .iter()
        .filter(|num| num.parse::<usize>().is_ok())
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    if nums.len() != 2 {
        return Err("Could not parse input to numbers");
    }
    Ok((nums[0], nums[1]))
}

fn ask_for_user_string(
    table: &'static [(&'static str, &str, &str)],
) -> Result<String, &'static str> {
    use std::io;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line when asking for user command");
    let input = input.trim();
    if input.len() == 2 {
        let option = choose_string_from_table(table, input);
        if option.is_none() {
            Err("String is of incorrect format.")
        } else {
            Ok(option.unwrap().to_string())
        }
    } else if input.chars().count() == 1 {
        Ok(input.to_owned())
    } else {
        Err("String is of incorrect format.")
    }
}

pub fn ask_and_run_command(mut config: &mut Config, mut board: &mut Board) {
    let mut cache = CommandCache::new();
    let mut command: Commands = Commands::Quit;
    loop {
        cache.print_current_page(config, board);

        match cache.current_page {
            Page::ConfigEmptyPiece
            | Page::EnterUserName1
            | Page::EnterUserName2
            | Page::EnterUserPiece1
            | Page::EnterUserPiece2
            | Page::Playing => {}
            _ => {
                command = Commands::keep_asking_for_valid_user_command();

                if command == Commands::Quit {
                    break;
                }
            }
        }

        println!("{}", Black.paint("=".repeat(51)));

        println!();

        cache
            .execute_command_and_update_command_cache(command, &mut config, &mut board)
            .unwrap_or_else(|err| {
                println!("{}", err);
            });
    }
}

enum Page {
    FrontPage,
    FrontPageHelp,
    Playing,
    ConfigMode,
    ConfigPlayer,
    ConfigFirst,
    ConfigAiSmartness,
    ConfigEmptyPiece,
    EnterUserName1,
    EnterUserName2,
    EnterUserPiece1,
    EnterUserPiece2,
    Winner,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Commands {
    Help,
    Start,
    Back,
    Quit,
    Command1,
    Command2,
    Command3,
    Command4,
    Command5,
    Command6,
    Restart,
    FrontPage,
}

impl Commands {
    fn ask_for_user_command() -> Option<Commands> {
        use std::io;
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line when asking for user command");
        let flag = input == "\n";
        let input = input.trim();
        if input == "h" || input == "help" {
            Some(Commands::Help)
        } else if input == "s" || input == "start" {
            Some(Commands::Start)
        } else if input == "q" || input == "quit" {
            Some(Commands::Quit)
        } else if input == "r" || input == "restart" {
            Some(Commands::Restart)
        } else if input == "f" || input == "frontpage" {
            Some(Commands::FrontPage)
        } else if input == "b" || input == "back" || flag {
            Some(Commands::Back)
        } else if input == "1" {
            Some(Commands::Command1)
        } else if input == "2" {
            Some(Commands::Command2)
        } else if input == "3" {
            Some(Commands::Command3)
        } else if input == "4" {
            Some(Commands::Command4)
        } else if input == "5" {
            Some(Commands::Command5)
        } else if input == "6" {
            Some(Commands::Command6)
        } else {
            None
        }
    }

    fn keep_asking_for_valid_user_command() -> Commands {
        let command: Commands;
        loop {
            command = match Commands::ask_for_user_command() {
                Some(command) => command,
                None => {
                    println!("unrecognized command");
                    continue;
                }
            };
            break;
        }
        command
    }
}

use std::fmt;
impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            Purple.paint(match &self {
                Commands::Help => "(h)elp",
                Commands::Back => "(b)ack/[Enter]",
                Commands::Start => "(s)tart",
                Commands::Quit => "(q)uit",
                Commands::Restart => "(r)estart",
                Commands::FrontPage => "(f)rontpage",
                Commands::Command1 => "(1)",
                Commands::Command2 => "(2)",
                Commands::Command3 => "(3)",
                Commands::Command4 => "(4)",
                Commands::Command5 => "(5)",
                Commands::Command6 => "(6)",
            })
        )
    }
}

use super::game::Players;
struct CommandCache {
    current_page: Page,
    available_commands: Vec<Commands>,
    current_player: Players,
    winner: Option<Players>,
}

impl CommandCache {
    fn new() -> CommandCache {
        CommandCache {
            current_page: Page::FrontPage,
            available_commands: FRONT_PAGE_AVAILABLE_COMMANDS.to_vec(),
            current_player: Players::Player1,
            winner: None,
        }
    }

    fn change_player(&mut self) {
        if self.current_player == Players::Player1 {
            self.current_player = Players::Player2;
        } else {
            self.current_player = Players::Player1;
        }
    }

    fn print_current_page(&self, config: &Config, board: &Board) {
        match self.current_page {
            Page::FrontPage => {
                print!(
                    "{}
Welcome to tictactoe-rust!

Current configuration: 
",
                    Cyan.paint(TICTACTOE_LOGO)
                );
                config.text_print_config(false);
                println!();
                self.print_available_commands();
            }
            Page::FrontPageHelp => {
                println!("{}", GAME_RULE);
                println!();
                self.print_available_commands();
            }
            Page::ConfigMode => {
                println!(
                    "Please enter mode:
  [1] PVP : User1 vs User2
  [2] PVC : User1 vs AI1
  [3] CVC : AI1   vs AI2
"
                );
                self.print_available_commands();
            }
            Page::ConfigPlayer => {
                println!("Configuration for name and piece character:");
                config.text_print_player_page_config();
                println!();
                self.print_available_commands();
            }
            Page::EnterUserName1 => {
                println!("Please enter new user name for Player1");
            }
            Page::EnterUserName2 => {
                println!("Please enter new user name for Player2");
            }
            Page::EnterUserPiece1 => {
                println!("Please enter new 1-len string for piece for Player1");
                println!();
                print_table(PIECE_TABLE, 2);
            }
            Page::EnterUserPiece2 => {
                println!("Please enter new 1-len string for piece for Player1");
                println!();
                print_table(PIECE_TABLE, 2);
            }
            Page::ConfigFirst => {
                println!(
                    "Please enter who will play first:
  [1] Player1
  [2] Player2
"
                );
                self.print_available_commands();
            }
            Page::ConfigAiSmartness => {
                println!(
                    "Choose the smartness level of Ai.

  [1] Kindergarden (random choose where is empty)
  [2] Elementary   (get match line if it can and step in your match line)
  [3] Graduate     (as smart as elementary)
  [4] God          (as smart as elementary)
"
                );
            }
            Page::ConfigEmptyPiece => {
                println!(
                    "Please enter the 1-len string for empty piece
Or choose one of the following:
"
                );
                print_table(EMPTY_PIECE_TABLE, 2);
            }
            Page::Playing => {
                super::game::Game::text_print_board(board, config);
            }
            Page::Winner => {
                super::game::Game::text_print_board(board, config);
                if self.winner.is_some() {
                    let winner = self.winner.unwrap();
                    let name = if Players::Player1 == winner {
                        config.players.0.name.as_str()
                    } else {
                        config.players.1.name.as_str()
                    };
                    println!("The winner is {}", name);
                } else {
                    println!("The game is a draw.");
                }
                board.print_matches(2);
                self.print_available_commands();
            }
        }
    }

    fn execute_command_and_update_command_cache(
        &mut self,
        command: Commands,
        config: &mut Config,
        board: &mut Board,
    ) -> Result<(), &str> {
        use super::game::{Mode, Players};
        match self.current_page {
            Page::FrontPage => match command {
                Commands::Help => {
                    self.current_page = Page::FrontPageHelp;
                    self.available_commands = FRONT_PAGE_HELP_AVAILABLE_COMMANDS.to_vec();
                    Ok(())
                }
                Commands::Start => {
                    self.current_page = Page::Playing;
                    self.available_commands = vec![];
                    Ok(())
                }
                Commands::Command1 => {
                    self.current_page = Page::ConfigMode;
                    self.available_commands = vec![
                        Commands::Command1,
                        Commands::Command2,
                        Commands::Command3,
                        Commands::Back,
                        Commands::Quit,
                    ];
                    Ok(())
                }
                Commands::Command2 | Commands::Command3 => {
                    self.current_page = Page::ConfigPlayer;
                    self.available_commands = CONFG_PLAYER_AVAILABLE_COMMANDS.to_vec();
                    Ok(())
                }
                Commands::Command4 => {
                    self.current_page = Page::ConfigFirst;
                    self.available_commands = vec![
                        Commands::Command1,
                        Commands::Command2,
                        Commands::Back,
                        Commands::Quit,
                    ];
                    Ok(())
                }
                Commands::Command5 => {
                    self.current_page = Page::ConfigAiSmartness;
                    self.available_commands = vec![
                        Commands::Command1,
                        Commands::Command2,
                        Commands::Command3,
                        Commands::Command4,
                        Commands::Back,
                        Commands::Quit,
                    ];
                    Ok(())
                }
                Commands::Command6 => {
                    self.current_page = Page::ConfigEmptyPiece;
                    self.available_commands = vec![];
                    Ok(())
                }
                Commands::Quit => {
                    panic!("Should not reach here");
                }
                _ => Err("Does not support this command on this page"),
            },
            Page::FrontPageHelp => match command {
                Commands::Back => {
                    self.current_page = Page::FrontPage;
                    self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                    Ok(())
                }
                Commands::Start => {
                    self.current_page = Page::Playing;
                    self.available_commands = vec![];
                    Ok(())
                }
                Commands::Quit => {
                    panic!("Should not reach here");
                }
                _ => Err("Does not support this command on this page"),
            },
            Page::ConfigMode => match command {
                Commands::Command1 => {
                    self.current_page = Page::FrontPage;
                    self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                    config.mode = Mode::PVP;
                    config.players.0.is_ai = false;
                    config.players.0.name = String::from("User1");
                    config.players.1.name = String::from("User2");
                    config.players.1.is_ai = false;
                    Ok(())
                }
                Commands::Command2 => {
                    self.current_page = Page::FrontPage;
                    self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                    config.mode = Mode::PVC;
                    config.players.0.is_ai = false;
                    config.players.1.is_ai = true;
                    config.players.0.name = String::from("User1");
                    config.players.1.name = String::from("AI1");
                    Ok(())
                }
                Commands::Command3 => {
                    self.current_page = Page::FrontPage;
                    self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                    config.mode = Mode::CVC;
                    config.players.0.is_ai = true;
                    config.players.1.is_ai = true;
                    config.players.0.name = String::from("AI1");
                    config.players.1.name = String::from("AI2");
                    Ok(())
                }
                Commands::Back => {
                    self.current_page = Page::FrontPage;
                    self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                    Ok(())
                }
                Commands::Quit => {
                    panic!("Should not reach here");
                }
                _ => Err("Does not support this command on this page"),
            },
            Page::ConfigPlayer => match command {
                Commands::Command1 => {
                    self.current_page = Page::EnterUserName1;
                    self.available_commands = vec![];
                    Ok(())
                }
                Commands::Command2 => {
                    self.current_page = Page::EnterUserName2;
                    self.available_commands = vec![];
                    Ok(())
                }
                Commands::Command3 => {
                    self.current_page = Page::EnterUserPiece1;
                    self.available_commands = vec![];
                    Ok(())
                }
                Commands::Command4 => {
                    self.current_page = Page::EnterUserPiece2;
                    self.available_commands = vec![];
                    Ok(())
                }
                Commands::Back => {
                    self.current_page = Page::FrontPage;
                    self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                    Ok(())
                }
                Commands::Quit => {
                    panic!("Should not reach here");
                }
                _ => Err("Does not support this command on this page"),
            },
            Page::EnterUserName1 => {
                self.current_page = Page::ConfigPlayer;
                self.available_commands = CONFG_PLAYER_AVAILABLE_COMMANDS.to_vec();
                config.players.0.name = ask_for_user_name();
                Ok(())
            }
            Page::EnterUserName2 => {
                self.current_page = Page::ConfigPlayer;
                self.available_commands = CONFG_PLAYER_AVAILABLE_COMMANDS.to_vec();
                config.players.1.name = ask_for_user_name();
                Ok(())
            }
            Page::EnterUserPiece1 => {
                self.current_page = Page::ConfigPlayer;
                self.available_commands = CONFG_PLAYER_AVAILABLE_COMMANDS.to_vec();
                let result = ask_for_user_string(PIECE_TABLE);
                if result.is_ok() {
                    let user_string = result.unwrap();
                    config.players.0.piece = user_string;
                    Ok(())
                } else {
                    Err(result.unwrap_err())
                }
            }
            Page::EnterUserPiece2 => {
                self.current_page = Page::ConfigPlayer;
                self.available_commands = CONFG_PLAYER_AVAILABLE_COMMANDS.to_vec();
                let result = ask_for_user_string(PIECE_TABLE);
                if result.is_ok() {
                    let user_string = result.unwrap();
                    config.players.1.piece = user_string;
                    Ok(())
                } else {
                    Err(result.unwrap_err())
                }
            }
            Page::ConfigFirst => match command {
                Commands::Command1 => {
                    self.current_page = Page::FrontPage;
                    self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                    config.first = Players::Player1;
                    self.current_player = Players::Player1;
                    Ok(())
                }
                Commands::Command2 => {
                    self.current_page = Page::FrontPage;
                    self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                    config.first = Players::Player2;
                    self.current_player = Players::Player2;
                    Ok(())
                }
                Commands::Back => {
                    self.current_page = Page::FrontPage;
                    self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                    Ok(())
                }
                Commands::Quit => {
                    panic!("Should not reach here");
                }
                _ => Err("Does not support this command on this page"),
            },
            Page::ConfigAiSmartness => {
                use super::game::SmartLevel;
                match command {
                    Commands::Command1 => {
                        config.ai_smartness = SmartLevel::Kindergarden;
                        self.current_page = Page::FrontPage;
                        self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                        Ok(())
                    }
                    Commands::Command2 => {
                        config.ai_smartness = SmartLevel::Elementary;
                        self.current_page = Page::FrontPage;
                        self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                        Ok(())
                    }
                    Commands::Command3 => {
                        config.ai_smartness = SmartLevel::Graduate;
                        self.current_page = Page::FrontPage;
                        self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                        Ok(())
                    }
                    Commands::Command4 => {
                        config.ai_smartness = SmartLevel::God;
                        self.current_page = Page::FrontPage;
                        self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                        Ok(())
                    }
                    Commands::Back => {
                        self.current_page = Page::FrontPage;
                        self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                        Ok(())
                    }
                    Commands::Quit => {
                        panic!("Should not reach here");
                    }
                    _ => Err("Does not support this command on this page"),
                }
            }
            Page::ConfigEmptyPiece => {
                let result = ask_for_user_string(EMPTY_PIECE_TABLE);
                if result.is_ok() {
                    let user_string = result.unwrap();
                    config.empty_piece = user_string;
                    self.current_page = Page::FrontPage;
                    self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                    Ok(())
                } else {
                    Err(result.unwrap_err())
                }
            }
            Page::Playing => {
                let mut coordinates: (usize, usize);
                use super::ai;
                if self.current_player == Players::Player1 && config.players.0.is_ai {
                    let coordinates =
                        ai::Ai::ask_ai_move_input(board, self.current_player, config.ai_smartness);
                    println!(
                        "{}'s input is ({}, {})",
                        config.players.0.name, coordinates.0, coordinates.1
                    );
                    board
                        .set_coordinate(coordinates.0, coordinates.1, self.current_player)
                        .unwrap();
                } else if self.current_player == Players::Player2 && config.players.1.is_ai {
                    let coordinates =
                        ai::Ai::ask_ai_move_input(board, self.current_player, config.ai_smartness);
                    println!(
                        "{}'s input is ({}, {})",
                        config.players.1.name, coordinates.0, coordinates.1
                    );
                    board
                        .set_coordinate(coordinates.0, coordinates.1, self.current_player)
                        .unwrap();
                } else {
                    loop {
                        let result = ask_user_move_input();
                        if result.is_ok() {
                            coordinates = result.unwrap();
                            let tmp_result = board.set_coordinate(
                                coordinates.0,
                                coordinates.1,
                                self.current_player,
                            );
                            if tmp_result.is_err() {
                                println!("{}", tmp_result.unwrap_err());
                                continue;
                            } else {
                                break;
                            }
                        } else {
                            let err = result.unwrap_err();
                            println!("{}", err);
                            if err == "Quit" {
                                std::process::exit(0);
                            }
                            continue;
                        }
                    }
                }

                board.update_matches();
                let player = board.has_match_line();
                if player.is_some() {
                    self.winner = player;
                    self.current_page = Page::Winner;
                    self.available_commands =
                        vec![Commands::Restart, Commands::FrontPage, Commands::Quit];
                } else if board.is_full() {
                    self.winner = None;
                    self.current_page = Page::Winner;
                    self.available_commands =
                        vec![Commands::Restart, Commands::FrontPage, Commands::Quit];
                } else {
                    self.change_player();
                }
                Ok(())
            }
            Page::Winner => match command {
                Commands::Restart => {
                    self.current_page = Page::Playing;
                    self.available_commands = vec![];
                    board.clear();
                    Ok(())
                }
                Commands::FrontPage => {
                    self.current_page = Page::FrontPage;
                    self.available_commands = FRONT_PAGE_AVAILABLE_COMMANDS.to_vec();
                    board.clear();
                    Ok(())
                }
                Commands::Quit => {
                    panic!("Should not reach here");
                }
                _ => Err("Does not support this command on this page"),
            },
        }
    }

    fn print_available_commands(&self) {
        use ansi_term::Colour::*;
        println!("{}", Blue.paint("Available commands:"));
        let mut iter = self.available_commands.iter();
        print!("{}", iter.next().unwrap());
        for c in iter {
            print!(" {}", c);
        }
        println!();
    }
}

#[cfg(test)]
mod tests;
