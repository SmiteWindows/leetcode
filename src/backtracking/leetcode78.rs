// https://leetcode.com/problems/subsets/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(
        start: usize,
        curr: &mut Vec<i32>,
        nums: &Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        k: usize,
        len: usize,
    ) {
        if curr.len() == k {
            return res.push(curr.to_vec());
        }
        for i in start..len {
            curr.push(nums[i]);
            backtrack(i + 1, curr, nums, res, k, len);
            curr.pop();
        }
    }
    let mut res = vec![vec![]];
    let mut curr = vec![];
    let n = nums.len();
    for k in 1..=n {
        backtrack(0, &mut curr, &nums, &mut res, k, n);
    }
    res
}
// array backtracking bit_manipulation
#[test]
fn test_2_78() {
    let nums = vec![1];
    let res = vec![vec![], vec![1]];
    assert_eq!(subsets(nums), res);
    let nums = vec![1, 2, 3];
    let res = vec![
        vec![],
        vec![1],
        vec![2],
        vec![3],
        vec![1, 2],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2, 3],
    ];
    assert_eq!(subsets(nums), res);
}
