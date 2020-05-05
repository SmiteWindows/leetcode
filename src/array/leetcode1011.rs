// https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/
pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
    todo!()
}
// binary_search array
#[test]
#[ignore]
fn test2_1011() {
    assert_eq!(ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
    assert_eq!(ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
    assert_eq!(ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
}
