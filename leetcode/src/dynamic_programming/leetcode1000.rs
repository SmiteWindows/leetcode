// https://leetcode-cn.com/problems/minimum-cost-to-merge-stones/
pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1000() {
    assert_eq!(merge_stones(vec![3, 2, 4, 1], 2), 20);
    assert_eq!(merge_stones(vec![3, 2, 4, 1], 3), -1);
    assert_eq!(merge_stones(vec![3, 5, 1, 2, 6], 3), 25);
}
