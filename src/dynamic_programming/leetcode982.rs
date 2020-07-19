// https://leetcode.com/problems/triples-with-bitwise-and-equal-to-zero/
// Runtime: 168 ms
// Memory Usage: 4.1 MB
use std::collections::HashMap;
pub fn count_triplets(a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut hm: HashMap<i32, usize> = HashMap::new();
    for i in 0..n {
        for j in 0..n {
            *hm.entry(a[i] & a[j]).or_default() += 1;
        }
    }
    let mut res = 0;
    for i in 0..n {
        for (&k, &v) in hm.iter() {
            if a[i] & k == 0 {
                res += v;
            }
        }
    }
    res as i32
}
// dynamic_programming
#[test]
fn test1_982() {
    assert_eq!(count_triplets(vec![2, 1, 3]), 12);
}
