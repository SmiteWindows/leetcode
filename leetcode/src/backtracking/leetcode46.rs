// https://leetcode-cn.com/problems/permutations/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
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
    use leetcode_prelude::vec2;
    assert_eq!(
        permute(vec![1, 2, 3]),
        vec2![
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 2, 1],
            [3, 1, 2]
        ]
    );
}
