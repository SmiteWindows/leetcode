// https://leetcode.com/problems/3sum/
// Runtime: 16 ms
// Memory Usage: 3.4 MB
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let n = nums.len();
    if n < 3 {
        return res;
    }
    let mut nums = nums;
    nums.sort_unstable();
    for i in 0..n - 2 {
        let a = nums[i];
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        if nums[i] > 0 {
            break;
        }
        let mut j = i + 1;
        let mut k = n - 1;
        while j < k {
            let b = nums[j];
            let c = nums[k];
            let sum = a + b + c;
            if sum == 0 {
                res.push(vec![a, b, c]);
                j += 1;
                k -= 1;
                while j < k && nums[j] == nums[j - 1] {
                    j += 1;
                }
                while j < k && nums[k] == nums[k + 1] {
                    k -= 1;
                }
            } else {
                if sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
    }
    res
}
// array two_pointers
#[test]
fn test1_15() {
    assert_eq!(
        three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, 0, 1], vec![-1, -1, 2]]
    );
}
