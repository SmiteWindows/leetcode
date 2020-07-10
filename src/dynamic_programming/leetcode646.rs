// https://leetcode.com/problems/maximum-length-of-pair-chain/
// Runtime: 8 ms
// Memory Usage: 2.2 MB
pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
    let mut pairs = pairs;
    let n = pairs.len();
    pairs.sort_by_key(|p| p[1]);
    let mut res = 0;
    let mut cur = i32::MIN;
    for i in 0..n {
        if cur < pairs[i][0] {
            cur = pairs[i][1];
            res += 1;
        }
    }
    res
}
// dynamic_programming
#[test]
fn test1_646() {
    assert_eq!(
        find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
        2
    );
}
