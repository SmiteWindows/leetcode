// https://leetcode.com/problems/sum-of-all-odd-length-subarrays/
pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    let n = arr.len() as i32;
    let mut res = 0;
    for i in 0..n {
        res += ((i + 1) * (n - i) + 1) / 2 * arr[i as usize];
    }
    res
}
// Runtime: 0 ms
// Memory Usage: 2.1 MB
// array
#[test]
fn test1_1588() {
    assert_eq!(sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
    assert_eq!(sum_odd_length_subarrays(vec![1, 2]), 3);
    assert_eq!(sum_odd_length_subarrays(vec![10, 11, 12]), 66);
}
