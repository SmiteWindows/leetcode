// https://leetcode-cn.com/problems/stamping-the-sequence/
pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
    todo!()
}
// greedy string
#[test]
#[ignore]
fn test1_936() {
    assert_eq!(
        moves_to_stamp("abc".to_string(), "ababc".to_string()),
        vec![0, 2]
    );
    assert_eq!(
        moves_to_stamp("abca".to_string(), "aabcaca".to_string()),
        vec![3, 0, 1]
    );
}
