// https://leetcode.com/problems/subsets/
/// Runtime: 0 ms
/// Memory Usage: 2.1 MB
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    if nums.is_empty() {
        return vec![vec![]];
    }
    let n = nums.pop().expect("exist");
    let mut res = subsets(nums);
    let len = res.len();
    for i in 0..len {
        res.push(res[i].clone());
        res.last_mut().expect("exist").push(n);
    }
    res
}
//  array backtracking bit_manipulation
#[test]
fn test1_78() {
    assert_eq!(subsets(vec![1]), vec![vec![], vec![1]]);
    assert_eq!(
        subsets(vec![1, 2, 3]),
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ]
    );
}
