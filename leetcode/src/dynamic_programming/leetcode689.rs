// https://leetcode-cn.com/problems/maximum-sum-of-3-non-overlapping-subarrays/
// Runtime: 12 ms
// Memory Usage: 2.8 MB
pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let sums: Vec<i32> = nums.windows(k).map(|w| w.iter().sum()).collect();
    let n = sums.len();
    let mut left = Vec::new();
    let mut left_max = 0;
    let mut left_index = 0;
    for (i, &sum) in sums.iter().enumerate().take(n) {
        if sums[i] > left_max {
            left_max = sum;
            left_index = i;
        }
        left.push((left_max, left_index));
    }

    let mut right = Vec::new();
    let mut right_max = 0;
    let mut right_index = n;
    for i in (0..n).rev() {
        if sums[i] >= right_max {
            right_max = sums[i];
            right_index = i;
        }
        right.push((right_max, right_index));
    }
    right.reverse();
    let mut mid_max = 0;
    let mut res = vec![0, 0, 0];
    for i in k..n - k {
        let sum_3 = sums[i] + left[i - k].0 + right[i + k].0;
        if sum_3 > mid_max {
            mid_max = sum_3;
            res = vec![left[i - k].1, i, right[i + k].1];
        }
    }
    res.into_iter().map(|x| x as i32).collect()
}
// dynamic_programming array
#[test]
fn test1_689() {
    assert_eq!(
        max_sum_of_three_subarrays(vec![1, 2, 1, 2, 6, 7, 5, 1], 2),
        vec![0, 3, 5]
    );
}
