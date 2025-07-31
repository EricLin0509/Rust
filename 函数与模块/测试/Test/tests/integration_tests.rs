use Test::add;

mod common;

#[test]
fn test_add() {
    common::setup();
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(10, 0), 10);
    assert_eq!(add(-1, -2), -3);
}