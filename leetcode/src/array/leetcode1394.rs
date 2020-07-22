// https://leetcode.com/problems/find-lucky-integer-in-an-array/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn find_lucky(arr: Vec<i32>) -> i32 {
    let mut hm = HashMap::new();
    for x in arr {
        *hm.entry(x).or_default() += 1;
    }
    if let Some(x) = hm
        .into_iter()
        .filter_map(|(k, v)| if k == v { Some(k) } else { None })
        .max()
    {
        x
    } else {
        -1
    }
}
// array
#[test]
fn test1_1394() {
    assert_eq!(find_lucky(vec![2, 2, 3, 4]), 2);
    assert_eq!(find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    assert_eq!(find_lucky(vec![2, 2, 2, 3, 3]), -1);
    assert_eq!(find_lucky(vec![5]), -1);
    assert_eq!(find_lucky(vec![7, 7, 7, 7, 7, 7, 7]), 7);
}
