use super::super::game::Players;
use super::*;
#[test]
fn ai_generate_random_valid_tuple_test() {
    let mut board: Board = Default::default();
    board.set_coordinate(0, 0, Players::Player1).unwrap();
    board.set_coordinate(1, 0, Players::Player2).unwrap();
    //board.set_coordinate(2, 0, Players::Player1).unwrap();
    //board.set_coordinate(0, 1, Players::Player2).unwrap();
    board.set_coordinate(1, 1, Players::Player1).unwrap();
    board.set_coordinate(2, 1, Players::Player2).unwrap();
    board.set_coordinate(0, 2, Players::Player2).unwrap();
    //board.set_coordinate(1, 2, Players::Player1).unwrap();
    board.set_coordinate(2, 2, Players::Player1).unwrap();

    let moves = Ai::ask_ai_move_input(&board, Players::Player1);
    println!("ai's move is ({}, {})", moves.0, moves.1);
    assert!(board.is_full());
    assert!(true);
}
