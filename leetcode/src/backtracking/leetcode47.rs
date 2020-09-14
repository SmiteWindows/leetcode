// https://leetcode-cn.com/problems/permutations-ii/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let n = nums.len();
    let mut res = vec![];
    let mut used = vec![false; n];
    let mut cur = vec![];
    nums.sort_unstable();
    dfs(&nums, &mut res, &mut used, &mut cur, n);
    res
}

fn dfs(nums: &[i32], res: &mut Vec<Vec<i32>>, used: &mut Vec<bool>, cur: &mut Vec<i32>, n: usize) {
    if cur.len() == n {
        res.push(cur.to_vec());
    } else {
        for i in 0..n {
            if used[i] {
                continue;
            }
            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue;
            }
            used[i] = true;
            cur.push(nums[i]);
            dfs(nums, res, used, cur, n);
            used[i] = false;
            cur.pop();
        }
    }
}
// backtracking
#[test]
fn test1_47() {
    use leetcode_prelude::vec2;
    assert_eq!(
        permute_unique(vec![1, 1, 2]),
        vec2![[1, 1, 2], [1, 2, 1], [2, 1, 1]]
    );
}
