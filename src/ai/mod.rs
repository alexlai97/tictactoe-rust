extern crate rand;

use super::board::Board;
use super::game::{Players, SmartLevel};
use rand::Rng;

pub struct Ai;

impl Ai {
    pub fn ask_ai_move_input(board: &Board, player: Players, level: SmartLevel ) -> (usize, usize) {
        match level {
            SmartLevel::Kindergarden => {
                ai_generate_random_valid_tuple(board, player)
            }
            SmartLevel::Elementary | _ => {
                let available_to_win_tuples = get_avaiable_to_win_tuples(board, player);
                let opponent = match player {
                    Players::Player1 => Players::Player2,
                    Players::Player2 => Players::Player1,
                };
                let opponent_available_to_win_tuples = get_avaiable_to_win_tuples(board, opponent);
                if ! available_to_win_tuples.is_empty() {
                    available_to_win_tuples[0]
                } else if ! opponent_available_to_win_tuples.is_empty() {
                    opponent_available_to_win_tuples[0]
                } else {
                    ai_generate_random_valid_tuple(board, player)
                }
            }
        } 
    }
}

fn ai_generate_random_valid_tuple(board: &Board, player: Players) -> (usize, usize) {
    let v = get_available_tuples(board, player);
    let random_valid_index = rand::thread_rng().gen_range(0, v.len());
    v[random_valid_index]
}

fn get_avaiable_to_win_tuples(board: &Board, player: Players) -> Vec<(usize, usize)> {
    let v = get_available_tuples(board, player);
    let mut new_v: Vec<(usize, usize)>  = Vec::new();
    let get_the_other_two_player = |two: [(usize, usize);2]| {
        (board.board[tuple_to_index(two[0])], 
        board.board[tuple_to_index(two[1])])
    };
    let if_three_player_matches = |three: [Option<Players>;3]| {
        three[0] == three[1] && three[0] == three[2]
    };
    for coor in v.iter() {
        let other_two_player_row = get_the_other_two_player(get_the_other_two_on(LineType::Row, coor.clone()));
        let other_two_player_col = get_the_other_two_player(get_the_other_two_on(LineType::Col, coor.clone()));
        let mut flag_diag0 = false;
        let mut flag_diag1 = false;
        if coor.0 == coor.1 { // Diag0
            let other_two_player_diag0 = get_the_other_two_player(get_the_other_two_on(LineType::Diag0, coor.clone()));
            flag_diag0 = if_three_player_matches([other_two_player_diag0.0, other_two_player_diag0.1, Some(player)]);
        } else if 2-coor.0 == coor.1 { //Diag1
            let other_two_player_diag1 = get_the_other_two_player(get_the_other_two_on(LineType::Diag1, coor.clone()));
            flag_diag1 = if_three_player_matches([other_two_player_diag1.0, other_two_player_diag1.1, Some(player)]);
        }
        if if_three_player_matches([other_two_player_row.0, other_two_player_row.1, Some(player)]) 
            || if_three_player_matches([other_two_player_col.0, other_two_player_col.1, Some(player)]) 
                || flag_diag0 || flag_diag1
        {
            new_v.push(coor.clone());
        }
    }
    new_v
}

fn get_available_tuples(board: &Board, player: Players) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = Vec::with_capacity(9);
    for (i, cell) in board.board.iter().enumerate() {
        if cell.is_none() {
            v.push(index_to_tuple(i));
        }
    }
    v
}

fn index_to_tuple(index: usize) -> (usize, usize) {
    (index % 3, index / 3)
}

fn tuple_to_index(tuple: (usize, usize)) -> usize {
    tuple.1 * 3 + tuple.0
}

fn check_cell_is(board: &Board, cell: (usize, usize), player: Players) -> bool {
    let option = board.board[tuple_to_index(cell)];
    option.is_some() && option.unwrap() == player
}

fn get_the_other_two_on(linetype: LineType, cell: (usize, usize)) -> [(usize, usize); 2] {
    let get_two_other_num = |num| {
        match num {
            0 => [1,2],
            1 => [0,2],
            2 => [0,1],
            _ => {
                panic!("wrong num");
            },
        }
    };
    match linetype {
        LineType::Row => {
            let row_line = cell.1;
            let other_two = get_two_other_num(cell.0);
            [(other_two[0],row_line), (other_two[1],row_line)]
        },
        LineType::Col => {
            let col_line = cell.0;
            let other_two = get_two_other_num(cell.1);
            [(col_line,other_two[0]), (col_line, other_two[1])]
        },
        LineType::Diag0 => {
            assert_eq!(cell.0,cell.1);
            let row_line = cell.1;
            let other_two = get_two_other_num(cell.1);
            [(other_two[0], other_two[0]), (other_two[1], other_two[1])]
        },
        LineType::Diag1 => {
            assert_eq!(2-cell.0,cell.1);
            let row_line = cell.1;
            let other_two = get_two_other_num(cell.1);
            [(2-other_two[0], other_two[0]), (2-other_two[1], other_two[1])]
        },
    }
}

enum LineType {
    Row,
    Col,
    Diag0,
    Diag1,
}
#[cfg(test)]
mod tests;
