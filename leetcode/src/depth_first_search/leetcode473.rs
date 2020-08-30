// https://leetcode-cn.com/problems/matchsticks-to-square/
// Runtime: 12 ms
// Memory Usage: 2 MB
pub fn makesquare(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    let n = nums.len();
    if n == 0 {
        return false;
    }
    let sum = nums.iter().sum::<i32>();
    if sum % 4 != 0 {
        return false;
    }
    nums.sort_unstable_by_key(|&x| -x);
    let mut sides = vec![0; 4];
    dfs(0, &mut sides, &nums, sum / 4, n)
}

fn dfs(start: usize, sides: &mut Vec<i32>, nums: &[i32], sum: i32, n: usize) -> bool {
    if start == n {
        true
    } else {
        for i in 0..4 {
            if sides[i] + nums[start] > sum {
                continue;
            }
            sides[i] += nums[start];
            if dfs(start + 1, sides, nums, sum, n) {
                return true;
            }
            sides[i] -= nums[start];
        }
        false
    }
}
// depth_first_search
#[test]
fn test1_473() {
    assert_eq!(makesquare(vec![1, 1, 2, 2, 2]), true);
    assert_eq!(makesquare(vec![3, 3, 3, 3, 4]), false);
}
