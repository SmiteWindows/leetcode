// https://leetcode-cn.com/problems/combination-sum/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort_unstable();
    let mut combination = Vec::new();
    let mut res = Vec::new();
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
        for (i, &candidate) in candidates.iter().enumerate().skip(begin) {
            if candidate > target {
                break;
            } else {
                combination.push(candidate);
                backtrack(res, combination, candidates, target - candidate, i);
                combination.pop();
            }
        }
    }
}
// array backtracking
#[test]
fn test2_39() {
    use leetcode_prelude::{assert_eq_sorted, vec2};
    assert_eq_sorted!(combination_sum(vec![2, 3, 6, 7], 7), vec2![[7], [2, 2, 3]]);
    assert_eq_sorted!(
        combination_sum(vec![2, 3, 5], 8),
        vec2![[2, 2, 2, 2], [2, 3, 3], [3, 5]]
    );
}
