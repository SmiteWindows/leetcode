// https://leetcode-cn.com/problems/combination-sum-iii/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    if k > 9 {
        return res;
    }
    let nums = (1..=9).collect::<Vec<_>>();
    let mut cur = vec![];
    dfs(0, n, &mut cur, &mut res, &nums, k as usize);
    res
}

fn dfs(
    start: usize,
    target: i32,
    cur: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
    nums: &[i32],
    n: usize,
) {
    if cur.len() == n && target == 0 {
        res.push(cur.to_vec());
    } else if target > 0 && start < nums.len() {
        dfs(start + 1, target, cur, res, nums, n);
        cur.push(nums[start]);
        dfs(start + 1, target - nums[start], cur, res, nums, n);
        cur.pop();
    }
}
// backtracking array
#[test]
fn test2_216() {
    use leetcode_prelude::vec2;
    assert_eq!(combination_sum3(3, 7), vec2![[1, 2, 4]]);
    assert_eq!(
        combination_sum3(3, 9),
        vec2![[2, 3, 4], [1, 3, 5], [1, 2, 6]]
    );
}
