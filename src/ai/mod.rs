extern crate rand;

use super::board::Board;
use super::game::Players;
use rand::Rng;

pub struct Ai;

impl Ai {
    pub fn ask_ai_move_input(board: &Board, player: Players) -> (usize, usize) {
        ai_generate_random_valid_tuple(board, player)
    }
}

fn ai_generate_random_valid_tuple(board: &Board, _player: Players) -> (usize, usize) {
    let mut v: Vec<usize> = Vec::with_capacity(9);
    for (i, cell) in board.board.iter().enumerate() {
        if cell.is_none() {
            v.push(i);
        }
    }
    let random_valid_index = rand::thread_rng().gen_range(0, v.len());
    index_to_tuple(v[random_valid_index])
}

fn index_to_tuple(index: usize) -> (usize, usize) {
    (index % 3, index / 3)
}

#[cfg(test)]
mod tests;
