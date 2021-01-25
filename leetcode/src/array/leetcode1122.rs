// https://leetcode-cn.com/problems/relative-sort-array/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{cmp::Ordering, collections::HashMap};
pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut hm = HashMap::new();
    for (i, &v) in arr2.iter().enumerate() {
        hm.insert(v, i);
    }
    arr1.sort_by(|a, b| match (hm.get(a), hm.get(b)) {
        (Some(i), Some(j)) => i.cmp(&j),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => a.cmp(&b),
    });
    arr1
}
// sort array
#[test]
fn test2_1122() {
    assert_eq!(
        relative_sort_array(
            vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
            vec![2, 1, 4, 3, 9, 6]
        ),
        vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
    );
}
