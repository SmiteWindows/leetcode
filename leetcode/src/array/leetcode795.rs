// https://leetcode-cn.com/problems/number-of-subarrays-with-bounded-maximum/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn num_subarray_bounded_max(a: Vec<i32>, l: i32, r: i32) -> i32 {
    let mut start = 0;
    let mut end = 0;
    let mut res = 0;
    for (i, &ai) in a.iter().enumerate() {
        if ai > r {
            start = i + 1;
        }
        if ai >= l {
            end = i + 1;
        }
        if start < end {
            res += end - start;
        }
    }
    res as i32
}
// array
#[test]
fn test1_795() {
    assert_eq!(num_subarray_bounded_max(vec![2, 1, 4, 3], 2, 3), 3);
}
