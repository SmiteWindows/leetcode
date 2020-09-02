// https://leetcode-cn.com/problems/combination-sum/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort_unstable();
    let mut combination = vec![];
    let mut res = vec![];
    backtrack(0, target, &mut combination, &mut res, &candidates);
    res
}

fn backtrack(
    start: usize,
    target: i32,
    combination: &mut Vec<i32>,
    all: &mut Vec<Vec<i32>>,
    candidates: &[i32],
) {
    if target == 0 {
        all.push(combination.to_vec());
    } else {
        for (i, &candidate) in candidates.iter().enumerate().skip(start) {
            if candidate > target {
                break;
            } else {
                combination.push(candidate);
                backtrack(i, target - candidates[i], combination, all, candidates);
                combination.pop();
            }
        }
    }
}
// array backtracking
#[test]
fn test1_39() {
    use leetcode_prelude::{assert_eq_sorted, vec2};
    assert_eq_sorted!(combination_sum(vec![2, 3, 6, 7], 7), vec2![[7], [2, 2, 3]]);
    assert_eq_sorted!(
        combination_sum(vec![2, 3, 5], 8),
        vec2![[2, 2, 2, 2], [2, 3, 3], [3, 5]]
    );
}
