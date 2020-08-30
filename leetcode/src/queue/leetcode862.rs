// https://leetcode-cn.com/problems/shortest-subarray-with-sum-at-least-k/
pub fn shortest_subarray(a: Vec<i32>, k: i32) -> i32 {
    todo!()
}
// binary_search queue
#[test]
#[ignore]
fn test1_862() {
    assert_eq!(shortest_subarray(vec![1], 1), 1);
    assert_eq!(shortest_subarray(vec![1, 2], 4), -1);
    assert_eq!(shortest_subarray(vec![2, -1, 2], 3), 3);
}
