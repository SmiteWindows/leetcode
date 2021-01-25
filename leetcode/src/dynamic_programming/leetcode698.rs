// https://leetcode-cn.com/problems/partition-to-k-equal-sum-subsets/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    let n = nums.len();
    let sum = nums.iter().sum::<i32>();
    if sum % k != 0 {
        return false;
    }
    let mut visited = vec![false; n];
    search(0, 0, k as usize, &mut visited, &nums, n, sum / k)
}

fn search(
    start: usize,
    sum: i32,
    k: usize,
    visited: &mut Vec<bool>,
    nums: &[i32],
    n: usize,
    target: i32,
) -> bool {
    if k == 0 {
        return true;
    }
    for i in start..n {
        if visited[i] {
            continue;
        }
        visited[i] = true;
        if sum + nums[i] < target && search(i + 1, sum + nums[i], k, visited, nums, n, target) {
            return true;
        }
        if sum + nums[i] == target && search(0, 0, k - 1, visited, nums, n, target) {
            return true;
        }
        visited[i] = false;
    }
    false
}
// dynamic_programming recursion
#[test]
fn test2_698() {
    assert_eq!(can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4), true);
}
