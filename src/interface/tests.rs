use super::*;
#[test]
fn print_table_test() {
    print_table(PIECE_TABLE, 2);
    assert!(true);
}

#[test]
fn choose_from_table_test() {
    assert_eq!(choose_string_from_table(PIECE_TABLE, "bp").unwrap(), "♟");
}
