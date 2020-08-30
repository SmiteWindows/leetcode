// https://leetcode-cn.com/problems/number-of-equivalent-domino-pairs/
// Runtime: 8 ms
// Memory Usage: 4.3 MB
use std::collections::HashMap;
pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut hs: HashMap<Vec<i32>, i32> = HashMap::new();
    let mut sum = 0;
    for d in dominoes {
        let a = d[0];
        let b = d[1];
        if a < b {
            *hs.entry(vec![a, b]).or_default() += 1;
        } else {
            *hs.entry(vec![b, a]).or_default() += 1;
        }
    }
    for v in hs.values() {
        sum += v * (v - 1) / 2;
    }
    sum
}
// array
#[test]
fn test1_1128() {
    assert_eq!(
        num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]),
        1
    );
}
