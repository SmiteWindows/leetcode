// https://leetcode-cn.com/problems/minimum-number-of-increments-on-subarrays-to-form-a-target-array/
// Runtime: 12 ms
// Memory Usage: 3.1 MB
pub fn min_number_operations(target: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut prev = 0;
    for x in target {
        if x > prev {
            res += x - prev;
        }
        prev = x;
    }
    res
}
// segment_tree
#[test]
fn test1_1526() {
    assert_eq!(min_number_operations(vec![1, 2, 3, 2, 1]), 3);
    assert_eq!(min_number_operations(vec![3, 1, 1, 2]), 4);
    assert_eq!(min_number_operations(vec![3, 1, 5, 4, 2]), 7);
    assert_eq!(min_number_operations(vec![1, 1, 1, 1]), 1);
}
