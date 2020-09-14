// https://leetcode-cn.com/problems/subsets/
/// Runtime: 0 ms
/// Memory Usage: 2.1 MB
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    if nums.is_empty() {
        return vec![vec![]];
    }
    let n = nums.pop().unwrap();
    let mut res = subsets(nums);
    let len = res.len();
    for i in 0..len {
        res.push(res[i].clone());
        res.last_mut().unwrap().push(n);
    }
    res
}
// array backtracking bit_manipulation
#[test]
fn test1_78() {
    use leetcode_prelude::vec2;
    assert_eq!(subsets(vec![1]), vec2![[], [1]]);
    assert_eq!(
        subsets(vec![1, 2, 3]),
        vec2![[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]]
    );
}
