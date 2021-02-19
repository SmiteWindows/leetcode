// https://leetcode-cn.com/problems/k-th-symbol-in-grammar/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn kth_grammar(n: i32, k: i32) -> i32 {
    let n = n as usize - 1;
    let k = k as usize - 1;
    kth(n, k)
}

fn kth(n: usize, k: usize) -> i32 {
    if n == 0 {
        0
    } else {
        kth(n - 1, k / 2) ^ (k % 2) as i32
    }
}
// recursion
#[test]
fn test1_779() {
    assert_eq!(kth_grammar(1, 1), 0);
    assert_eq!(kth_grammar(2, 1), 0);
    assert_eq!(kth_grammar(2, 2), 1);
    assert_eq!(kth_grammar(4, 5), 1);
}
