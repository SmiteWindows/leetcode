// https://leetcode.com/problems/maximum-sum-of-two-non-overlapping-subarrays/
pub fn max_sum_two_no_overlap(a: Vec<i32>, l: i32, m: i32) -> i32 {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1031() {
    assert_eq!(
        max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2),
        20
    );
    assert_eq!(
        max_sum_two_no_overlap(vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2),
        29
    );
    assert_eq!(
        max_sum_two_no_overlap(vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3),
        31
    );
}
