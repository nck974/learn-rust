use chapter_11_tests;

mod common;

#[test]
fn integration_test(){
    common::setup();
    assert_eq!(chapter_11_tests::add_two(2), 4)
}