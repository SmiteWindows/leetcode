// https://leetcode.com/problems/maximum-sum-obtained-of-any-permutation/
pub fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_1589() {
    use leetcode_prelude::vec2;
    assert_eq!(
        max_sum_range_query(vec![1, 2, 3, 4, 5], vec2![[1, 3], [0, 1]]),
        19
    );
    assert_eq!(
        max_sum_range_query(vec![1, 2, 3, 4, 5, 6], vec2![[0, 1]]),
        11
    );
    assert_eq!(
        max_sum_range_query(vec![1, 2, 3, 4, 5, 10], vec2![[0, 2], [1, 3], [1, 1]]),
        47
    );
}
