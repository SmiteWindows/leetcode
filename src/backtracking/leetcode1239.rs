// https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/
pub fn max_length(arr: Vec<String>) -> i32 {
    todo!()
}
// backtracking bit_manipulation
#[test]
#[ignore]
fn test1_1239() {
    assert_eq!(
        max_length(vec![
            String::from("un"),
            String::from("iq"),
            String::from("ue")
        ]),
        4
    );
    assert_eq!(
        max_length(vec![
            String::from("cha"),
            String::from("r"),
            String::from("act"),
            String::from("ers")
        ]),
        6
    );
    assert_eq!(
        max_length(vec![String::from("abcdefghijklmnopqrstuvwxyz")]),
        26
    );
}
