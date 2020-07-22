// https://leetcode.com/problems/partition-array-into-disjoint-intervals/
// Runtime: 8 ms
// Memory Usage: 2.4 MB
pub fn partition_disjoint(a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for i in (0..n).rev() {
        min = min.min(a[i]);
        right[i] = min;
    }

    for i in 0..n {
        max = max.max(a[i]);
        left[i] = max;
    }
    for i in 1..n {
        if right[i] >= left[i - 1] {
            return i as i32;
        }
    }
    0
}
// array
#[test]
fn test1_915() {
    assert_eq!(partition_disjoint(vec![5, 0, 3, 8, 6]), 3);
    assert_eq!(partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
}
