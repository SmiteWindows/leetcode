// https://leetcode.com/problems/combination-sum/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort_unstable();
    let mut combination = vec![];
    let mut res = vec![];
    backtrack(&mut res, &mut combination, &candidates, target, 0);
    res
}

fn backtrack(
    res: &mut Vec<Vec<i32>>,
    combination: &mut Vec<i32>,
    candidates: &[i32],
    target: i32,
    begin: usize,
) {
    if target == 0 {
        res.push(combination.to_vec());
    } else {
        for i in begin..candidates.len() {
            if candidates[i] > target {
                break;
            } else {
                combination.push(candidates[i]);
                backtrack(res, combination, candidates, target - candidates[i], i);
                combination.pop();
            }
        }
    }
}
// array backtracking
#[test]
fn test2_39() {
    assert_eq!(
        combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7]]
    );
    assert_eq!(
        combination_sum(vec![2, 3, 5], 8),
        vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
    );
}
