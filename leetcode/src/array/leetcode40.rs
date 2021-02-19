// https://leetcode-cn.com/problems/combination-sum-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut combination = Vec::new();
    let mut res = Vec::new();
    candidates.sort_unstable();
    dfs(&candidates, &mut combination, &mut res, target, 0);
    res
}

fn dfs(
    candidates: &[i32],
    combination: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
    target: i32,
    start: usize,
) {
    use std::cmp::Ordering::{Equal, Greater, Less};
    match target.cmp(&0) {
        Equal => {
            res.push(combination.to_vec());
        }
        Greater => {
            for (i, &candidate) in candidates.iter().enumerate().skip(start) {
                if candidate > target {
                    break;
                }
                if i > start && candidate == candidates[i - 1] {
                    continue;
                }
                combination.push(candidate);
                dfs(candidates, combination, res, target - candidate, i + 1);
                combination.pop();
            }
        }
        Less => {}
    }
}
// array backtracking
#[test]
fn test2_40() {
    use leetcode_prelude::{assert_eq_sorted, vec2};
    assert_eq_sorted!(
        combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
        vec2![[1, 7], [1, 2, 5], [2, 6], [1, 1, 6]]
    );
    assert_eq_sorted!(
        combination_sum2(vec![2, 5, 2, 1, 2], 5),
        vec2![[1, 2, 2], [5]]
    );
}
