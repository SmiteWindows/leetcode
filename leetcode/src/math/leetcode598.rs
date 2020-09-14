// https://leetcode-cn.com/problems/range-addition-ii/
// Runtime: 0 ms
// Memory Usage: 2.6 MB
pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
    let mut m = m;
    let mut n = n;
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
