// https://leetcode.com/problems/stamping-the-sequence/
pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
    todo!()
}
// greedy string
#[test]
#[ignore]
fn test2_936() {
    assert_eq!(
        moves_to_stamp(String::from("abc"), String::from("ababc")),
        vec![0, 2]
    );
    assert_eq!(
        moves_to_stamp(String::from("abca"), String::from("aabcaca")),
        vec![3, 0, 1]
    );
}
