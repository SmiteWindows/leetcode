// https://leetcode-cn.com/problems/increasing-subsequences/
// Runtime: 36 ms
// Memory Usage: 5 MB
pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let n = nums.len();
    let mut cur = Vec::new();
    dfs(0, &mut cur, &mut res, &nums, n);
    res
}

fn dfs(start: usize, cur: &mut Vec<i32>, all: &mut Vec<Vec<i32>>, nums: &[i32], n: usize) {
    if start == n {
        if cur.len() > 1 {
            all.push(cur.to_vec());
        }
    } else {
        if cur.is_empty() || nums[start] >= *cur.last().unwrap() {
            cur.push(nums[start]);
            dfs(start + 1, cur, all, nums, n);
            cur.pop();
        }
        if cur.is_empty() || nums[start] != *cur.last().unwrap() {
            dfs(start + 1, cur, all, nums, n);
        }
    }
}
// depth_first_search
#[test]
fn test1_491() {
    use leetcode_prelude::{assert_eq_sorted, vec2};
    assert_eq_sorted!(
        find_subsequences(vec![4, 6, 7, 7]),
        vec2![
            [4, 6],
            [4, 7],
            [4, 6, 7],
            [4, 6, 7, 7],
            [6, 7],
            [6, 7, 7],
            [7, 7],
            [4, 7, 7]
        ],
    );
}
