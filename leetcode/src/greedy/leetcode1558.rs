// https://leetcode-cn.com/problems/minimum-numbers-of-function-calls-to-make-target-array/
// Runtime: 12 ms
// Memory Usage: 3.1 MB
// ✔
pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut ones = 0;
    let mut max_width = 0;
    for mut x in nums {
        ones += x.count_ones();
        let mut width = 0;
        while x > 0 {
            x >>= 1;
            width += 1;
        }
        max_width = max_width.max(width);
    }
    (ones + max_width - 1) as i32
}
// greedy
#[test]
fn test1_1558() {
    assert_eq!(min_operations(vec![1, 5]), 5);
    assert_eq!(min_operations(vec![2, 2]), 3);
    assert_eq!(min_operations(vec![4, 2, 5]), 6);
    assert_eq!(min_operations(vec![3, 2, 2, 4]), 7);
    assert_eq!(min_operations(vec![2, 4, 8, 16]), 8);
}
use std::cmp::Reverse;
pub fn _min_operations(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable_by_key(|&x| Reverse(x));

    let mut count = 0;
    while !nums.is_empty() {
        let mut tmp = false;

        for num in &mut nums {
            if *num % 2 == 1 {
                *num -= 1;
                count += 1;
                tmp = true;
            }
        }
        if !tmp {
            count += 1;
            for num in &mut nums {
                *num /= 2;
            }
        }
        while !nums.is_empty() && nums[nums.len() - 1] == 0 {
            nums.pop();
        }
    }
    count
}
