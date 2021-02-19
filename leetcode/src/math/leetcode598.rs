// https://leetcode-cn.com/problems/range-addition-ii/
// Runtime: 0 ms
// Memory Usage: 2.6 MB
pub fn max_count(mut m: i32, mut n: i32, ops: Vec<Vec<i32>>) -> i32 {
    for op in ops {
        m = m.min(op[0]);
        n = n.min(op[1]);
    }
    m * n
}
// math
#[test]
fn test1_598() {
    use leetcode_prelude::vec2;
    assert_eq!(max_count(3, 3, vec2![[2, 2], [3, 3]]), 4);
}
