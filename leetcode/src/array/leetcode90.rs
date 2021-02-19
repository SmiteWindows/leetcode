// https://leetcode-cn.com/problems/subsets-ii/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let n = nums.len();
    let mut cur = Vec::new();
    let mut res = Vec::new();
    dfs(0, &mut cur, &mut res, &nums, n);
    res
}

fn dfs(start: usize, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, nums: &[i32], n: usize) {
    res.push(cur.to_vec());
    if start == n {
        return;
    }
    for (i, &num) in nums.iter().enumerate().skip(start) {
        if i > start && num == nums[i - 1] {
            continue;
        }
        cur.push(num);
        dfs(i + 1, cur, res, nums, n);
        cur.pop();
    }
}
// array backtracking
#[test]
fn test2_90() {
    use leetcode_prelude::{assert_eq_sorted, vec2};
    assert_eq_sorted!(
        subsets_with_dup(vec![1, 2, 2]),
        vec2![[2], [1], [1, 2, 2], [2, 2], [1, 2], []]
    );
}
