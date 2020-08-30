// https://leetcode-cn.com/problems/subsets/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![vec![]];
    let mut curr = vec![];
    let n = nums.len();
    for k in 1..=n {
        backtrack(0, &mut curr, &nums, &mut res, k);
    }
    res
}

fn backtrack(start: usize, curr: &mut Vec<i32>, nums: &[i32], res: &mut Vec<Vec<i32>>, k: usize) {
    if curr.len() == k {
        return res.push(curr.to_vec());
    }
    for (i, &num) in nums.iter().enumerate().skip(start) {
        curr.push(num);
        backtrack(i + 1, curr, nums, res, k);
        curr.pop();
    }
}
// array backtracking bit_manipulation
#[test]
fn test2_78() {
    assert_eq!(subsets(vec![1]), vec![vec![], vec![1]]);
    assert_eq!(
        subsets(vec![1, 2, 3]),
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ]
    );
}
