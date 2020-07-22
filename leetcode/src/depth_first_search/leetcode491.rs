// https://leetcode.com/problems/increasing-subsequences/
// Runtime: 36 ms
// Memory Usage: 5 MB
pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let n = nums.len();
    let mut cur = vec![];
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
    let nums = vec![4, 6, 7, 7];
    let mut res = vec![
        vec![4, 6],
        vec![4, 7],
        vec![4, 6, 7],
        vec![4, 6, 7, 7],
        vec![6, 7],
        vec![6, 7, 7],
        vec![7, 7],
        vec![4, 7, 7],
    ];
    let mut ans = find_subsequences(nums);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
