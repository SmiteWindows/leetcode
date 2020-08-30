// https://leetcode-cn.com/problems/132-pattern/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn find132pattern(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut a3 = i32::MIN;
    let mut stack = vec![];
    for i in (0..n).rev() {
        if nums[i] < a3 {
            return true;
        } else {
            while let Some(top) = stack.pop() {
                if nums[i] > top {
                    a3 = top;
                } else {
                    stack.push(top);
                    break;
                }
            }
        }
        stack.push(nums[i]);
    }
    false
}
// stack
#[test]
fn test1_456() {
    assert_eq!(find132pattern(vec![1, 2, 3, 4]), false);
    assert_eq!(find132pattern(vec![3, 1, 4, 2]), true);
    assert_eq!(find132pattern(vec![-1, 3, 2, 0]), true);
}
