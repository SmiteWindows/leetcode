// https://leetcode-cn.com/problems/maximum-length-of-pair-chain/
// Runtime: 8 ms
// Memory Usage: 2.2 MB
pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
    let mut pairs = pairs;
    pairs.sort_by_key(|p| p[1]);
    let mut res = 0;
    let mut cur = i32::MIN;
    for pair in pairs.iter() {
        if cur < pair[0] {
            cur = pair[1];
            res += 1;
        }
    }
    res
}
// dynamic_programming
#[test]
fn test1_646() {
    use leetcode_prelude::vec2;
    assert_eq!(find_longest_chain(vec2![[1, 2], [2, 3], [3, 4]]), 2);
}
