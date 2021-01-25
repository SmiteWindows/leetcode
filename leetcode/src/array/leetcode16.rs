// https://leetcode-cn.com/problems/3sum-closest/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut res = 0;
    let mut diff = i32::MAX;

    for i in 0..n - 2 {
        let mut l = i + 1;
        let mut r = n - 1;

        while l < r {
            let sum = nums[l] + nums[r] + nums[i];

            if sum == target {
                return target;
            }

            if (sum - target).abs() < diff {
                diff = (sum - target).abs();
                res = sum;
            }

            if nums[l] + nums[r] > target - nums[i] {
                r -= 1;
            } else {
                l += 1;
            }
        }
    }
    res
}
// array two_pointers
#[test]
fn test2_16() {
    assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
}
