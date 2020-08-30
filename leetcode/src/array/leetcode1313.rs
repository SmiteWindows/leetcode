// https://leetcode-cn.com/problems/decompress-run-length-encoded-list/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    for p in nums.chunks(2) {
        let a = p[0] as usize;
        let b = p[1];
        res.resize(res.len() + a, b);
    }
    res
}
// array
#[test]
fn test1_1313() {
    assert_eq!(decompress_rl_elist(vec![1, 2, 3, 4]), vec![2, 4, 4, 4]);
    assert_eq!(decompress_rl_elist(vec![1, 1, 2, 3]), vec![1, 3, 3]);
}
