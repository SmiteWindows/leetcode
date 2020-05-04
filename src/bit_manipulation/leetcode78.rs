// https://leetcode.com/problems/subsets/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();
    let all_lens = usize::pow(2usize, len as u32);
    let mut res = vec![vec![]; all_lens];
    for (i, num) in nums.iter().enumerate().take(len).map(|(i, &num)| (i, num)) {
        let mod_lens = usize::pow(2usize, (len - 1 - i) as u32);
        for (j, item) in res.iter_mut().enumerate().take(all_lens) {
            if (j / mod_lens) % 2 == 0 {
                item.push(num);
            }
        }
    }
    res
}
// array backtracking bit_manipulation
#[test]
fn test3_78() {
    assert_eq!(subsets(vec![1]), vec![vec![1], vec![]]);
    assert_eq!(
        subsets(vec![1, 2, 3]),
        vec![
            vec![1, 2, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1],
            vec![2, 3],
            vec![2],
            vec![3],
            vec![],
        ]
    );
}
