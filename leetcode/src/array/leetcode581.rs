// https://leetcode-cn.com/problems/shortest-unsorted-continuous-subarray/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut l = n - 1;
    let mut r = 0;

    for (i, &val) in nums.iter().enumerate() {
        max = max.max(val);
        if max != val {
            r = i;
        }
    }

    for (i, &val) in nums.iter().enumerate().rev() {
        min = min.min(val);
        if min != val {
            l = i;
        }
    }
    if r <= l {
        0
    } else {
        (r - l + 1) as i32
    }
}
// array
#[test]
fn test1_581() {
    assert_eq!(find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]), 5);
}
