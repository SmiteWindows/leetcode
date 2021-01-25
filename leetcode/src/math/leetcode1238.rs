// https://leetcode-cn.com/problems/circular-permutation-in-binary-representation/
// Runtime: 16 ms
// Memory Usage: 2.7 MB
pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
    let mut res = vec![];
    for i in 0..1 << n {
        res.push(start ^ (i ^ i >> 1));
    }
    res
}
// math
#[test]
fn test1_1238() {
    assert_eq!(circular_permutation(2, 3), vec![3, 2, 0, 1]);
    assert_eq!(circular_permutation(3, 2), vec![2, 3, 1, 0, 4, 5, 7, 6]);
}
