pub fn num_splits(s: String) -> i32 {
    todo!()
}
// string bit_manipulation
#[test]
#[ignore]
fn test2_1525() {
    assert_eq!(num_splits("aacaba".to_string()), 2);
    assert_eq!(num_splits("abcd".to_string()), 1);
    assert_eq!(num_splits("aaaaa".to_string()), 4);
    assert_eq!(num_splits("acbadbaada".to_string()), 2);
}
