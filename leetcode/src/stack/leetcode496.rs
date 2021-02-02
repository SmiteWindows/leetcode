// https://leetcode-cn.com/problems/next-greater-element-i/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::new();
    let mut hm = HashMap::new();
    for x in nums2 {
        while let Some(last) = stack.pop() {
            if last > x {
                stack.push(last);
                break;
            } else {
                hm.insert(last, x);
            }
        }
        stack.push(x);
    }
    nums1.iter().map(|x| *hm.get(x).unwrap_or(&-1)).collect()
}
// stack
#[test]
fn test1_496() {
    assert_eq!(
        next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
        vec![-1, 3, -1]
    );
    assert_eq!(
        next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
        vec![3, -1]
    );
}
