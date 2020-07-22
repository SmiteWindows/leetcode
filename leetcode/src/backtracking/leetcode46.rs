// https://leetcode.com/problems/permutations/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut res = vec![];
    let n = nums.len();
    backtrack(&mut nums, &mut res, 0, n);
    res
}

fn backtrack(nums: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, start: usize, length: usize) {
    if start == length {
        res.push(nums.to_vec());
        return;
    }
    for i in start..length {
        nums.swap(start, i);
        backtrack(nums, res, start + 1, length);
        nums.swap(start, i);
    }
}
// backtracking
#[test]
fn test1_46() {
    assert_eq!(
        permute(vec![1, 2, 3]),
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 2, 1],
            vec![3, 1, 2]
        ]
    );
}
