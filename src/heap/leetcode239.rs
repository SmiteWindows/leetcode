// https://leetcode.com/problems/sliding-window-maximum/
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    todo!()
}
// heap sliding_window
#[test]
#[ignore]
fn test2_239() {
    assert_eq!(
        max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );
}
