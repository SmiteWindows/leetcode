// https://leetcode-cn.com/problems/maximum-equal-frequency/
pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_1224() {
    assert_eq!(max_equal_freq(vec![2, 2, 1, 1, 5, 3, 3, 5]), 7);
    assert_eq!(
        max_equal_freq(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5]),
        13
    );
    assert_eq!(max_equal_freq(vec![1, 1, 1, 2, 2, 2]), 5);
    assert_eq!(max_equal_freq(vec![10, 2, 8, 9, 3, 8, 1, 5, 2, 3, 7, 6]), 8);
}
