// https://leetcode.com/problems/advantage-shuffle/
pub fn advantage_count(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    todo!()
}
// greedy array
#[test]
#[ignore]
fn test2_870() {
    assert_eq!(
        advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]),
        vec![2, 11, 7, 15]
    );
    assert_eq!(
        advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11]),
        vec![24, 32, 8, 12]
    );
}
