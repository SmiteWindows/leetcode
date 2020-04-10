// https://leetcode.com/problems/subsets/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();
    let all_lens = usize::pow(2usize, len as u32);
    let mut res = vec![vec![]; all_lens];
    for i in 0..len {
        let mod_lens = usize::pow(2usize, (len - 1 - i) as u32);
        let num = nums[i];
        for j in 0..all_lens {
            if (j / mod_lens) % 2 == 0 {
                res[j].push(num);
            }
        }
    }
    res
}
// array backtracking bit_manipulation
#[test]
fn test_3_78() {
    let nums = vec![1];
    let res = vec![vec![1], vec![]];
    assert_eq!(subsets(nums), res);
    let nums = vec![1, 2, 3];
    let res = vec![
        vec![1, 2, 3],
        vec![1, 2],
        vec![1, 3],
        vec![1],
        vec![2, 3],
        vec![2],
        vec![3],
        vec![],
    ];
    assert_eq!(subsets(nums), res);
}
