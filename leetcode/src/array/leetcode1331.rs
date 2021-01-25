// https://leetcode-cn.com/problems/rank-transform-of-an-array/
// Runtime: 32 ms
// Memory Usage: 5.5 MB
use std::collections::{BTreeSet, HashMap};
pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    let mut bts = BTreeSet::new();
    let mut hm = HashMap::new();
    for &x in &arr {
        bts.insert(x);
    }
    for (i, x) in bts.into_iter().enumerate() {
        hm.insert(x, i as i32 + 1);
    }
    arr.into_iter().map(|x| hm[&x]).collect()
}
// array
#[test]
fn test1_1331() {
    assert_eq!(array_rank_transform(vec![40, 10, 20, 30]), vec![4, 1, 2, 3]);
    assert_eq!(array_rank_transform(vec![100, 100, 100]), vec![1, 1, 1]);
    assert_eq!(
        array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
        vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
    );
}
