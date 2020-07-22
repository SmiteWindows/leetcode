// https://leetcode.com/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold/
// Runtime: 16 ms
// Memory Usage: 3.3 MB
pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
    let n = arr.len();
    let mut prefix = vec![0; n + 1];
    let sum = threshold * k;
    let k = k as usize;
    let mut res = 0;
    for i in 0..n {
        prefix[i + 1] = prefix[i] + arr[i];
    }
    for r in k..=n {
        let l = r - k;
        if prefix[r] - prefix[l] >= sum {
            res += 1;
        }
    }
    res
}
// array
#[test]
fn test1_1343() {
    assert_eq!(num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4), 3);
    assert_eq!(num_of_subarrays(vec![1, 1, 1, 1, 1], 1, 0), 5);
    assert_eq!(
        num_of_subarrays(vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5),
        6
    );
    assert_eq!(num_of_subarrays(vec![7, 7, 7, 7, 7, 7, 7], 7, 7), 1);
    assert_eq!(num_of_subarrays(vec![4, 4, 4, 4], 4, 1), 1);
}
