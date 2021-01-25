// https://leetcode-cn.com/problems/split-array-largest-sum/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
    let mut lo = *nums.iter().max().unwrap();
    let mut hi = nums.iter().sum();
    let n = nums.len();
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if split(&nums, mid, n) <= m {
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn split(nums: &[i32], max: i32, n: usize) -> i32 {
    let mut sum = 0;
    let mut res = 1;
    for &num in nums.iter().take(n) {
        if num + sum > max {
            sum = num;
            res += 1;
        } else {
            sum += num;
        }
    }
    res
}
// binary_search dynamic_programming
#[test]
fn test2_410() {
    assert_eq!(split_array(vec![7, 2, 5, 10, 8], 2), 18);
}
