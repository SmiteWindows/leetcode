// https://leetcode.com/problems/minimum-possible-integer-after-at-most-k-adjacent-swaps-on-digits/
pub fn min_integer(num: String, k: i32) -> String {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_1505() {
    assert_eq!(min_integer("4321".to_string(), 4), "1342".to_string());
    assert_eq!(min_integer("100".to_string(), 1), "010".to_string());
    assert_eq!(min_integer("36789".to_string(), 1000), "36789".to_string());
    assert_eq!(min_integer("22".to_string(), 22), "22".to_string());
    assert_eq!(
        min_integer("9438957234785635408".to_string(), 23),
        "0345989723478563548".to_string()
    );
}
