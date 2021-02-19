// https://leetcode-cn.com/problems/n-repeated-element-in-size-2n-array/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn repeated_n_times(a: Vec<i32>) -> i32 {
    let mut hs: HashSet<i32> = HashSet::new();
    for x in a {
        if !hs.insert(x) {
            return x;
        }
    }
    unreachable!()
}
// hash_table
#[test]
fn test1_961() {
    assert_eq!(repeated_n_times(vec![1, 2, 3, 3]), 3);
    assert_eq!(repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
    assert_eq!(repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
}
