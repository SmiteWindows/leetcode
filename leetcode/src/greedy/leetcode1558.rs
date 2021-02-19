// https://leetcode-cn.com/problems/minimum-numbers-of-function-calls-to-make-target-array/
pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut max = 0;

    for num in nums {
        max = max.max(num);
        res += num.count_ones();
    }

    if max > 0 {
        res += 31 - max.leading_zeros();
    }

    res as i32
}
// Runtime: 4 ms
// Memory Usage: 3 MB
// ✔✔
// greedy
#[test]
fn test1_1558() {
    assert_eq!(min_operations(vec![0]), 0);
    assert_eq!(min_operations(vec![1, 5]), 5);
    assert_eq!(min_operations(vec![2, 2]), 3);
    assert_eq!(min_operations(vec![4, 2, 5]), 6);
    assert_eq!(min_operations(vec![3, 2, 2, 4]), 7);
    assert_eq!(min_operations(vec![2, 4, 8, 16]), 8);
}
