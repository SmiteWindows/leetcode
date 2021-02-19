// https://leetcode-cn.com/problems/unique-binary-search-trees/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn num_trees(n: i32) -> i32 {
    let n = n as usize;
    let mut g = vec![0; n + 1];
    g[0] = 1;
    g[1] = 1;
    for i in 2..=n {
        for j in 1..=i {
            g[i] += g[j - 1] * g[i - j];
        }
    }
    g[n]
}
// tree dynamic_programming
#[test]
fn test2_96() {
    assert_eq!(num_trees(3), 5);
    assert_eq!(num_trees(19), 1767263190);
}
