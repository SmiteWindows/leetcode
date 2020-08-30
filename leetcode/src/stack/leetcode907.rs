// https://leetcode-cn.com/problems/sum-of-subarray-minimums/
// Runtime: 12 ms
// Memory Usage: 2.7 MB
pub fn sum_subarray_mins(a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut sum = 0;
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    let mut stack = vec![];
    for i in 0..n {
        left[i] = i + 1;
        right[i] = n - i;
        while let Some(j) = stack.pop() {
            if a[j] < a[i] {
                stack.push(j);
                break;
            } else {
                right[j] = i - j;
            }
        }
        if let Some(&j) = stack.last() {
            left[i] = i - j;
        }
        stack.push(i);
    }
    for i in 0..n {
        sum += (left[i] * right[i]) as i32 * a[i];
        sum %= 1_000_000_007;
    }
    sum
}
// stack array
#[test]
fn test1_907() {
    assert_eq!(sum_subarray_mins(vec![3, 1, 2, 4]), 17);
}
