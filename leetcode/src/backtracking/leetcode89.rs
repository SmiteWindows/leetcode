// https://leetcode-cn.com/problems/gray-code/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn gray_code(n: i32) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 0..(1 << n) {
        res.push(i ^ i >> 1);
    }
    res
}
// backtracking
#[test]
fn test1_89() {
    assert_eq!(gray_code(2), vec![0, 1, 3, 2]);
    assert_eq!(gray_code(0), vec![0]);
}
