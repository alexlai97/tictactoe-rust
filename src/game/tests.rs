use super::super::board::*;
use super::*;

#[test]
fn match_test() {
    let mut board: Board = Default::default();
    board.set_coordinate(0, 0, Players::Player1).unwrap();
    board.set_coordinate(1, 0, Players::Player2).unwrap();
    board.set_coordinate(2, 0, Players::Player2).unwrap();
    board.set_coordinate(0, 1, Players::Player1).unwrap();
    board.set_coordinate(0, 2, Players::Player1).unwrap();
    board.set_coordinate(1, 1, Players::Player1).unwrap();
    board.set_coordinate(2, 1, Players::Player1).unwrap();
    board.update_matches();
    board.print_matches(2);
    assert!(board.has_match_line() == Some(Players::Player1));

    let mut game: Game = Default::default();
    game.board = board;
    Game::text_print_board(&game.board, &game.config);
    assert!(true);
}
