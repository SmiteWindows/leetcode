// https://leetcode-cn.com/problems/next-greater-element-ii/
// Runtime: 8 ms
// Memory Usage: 2.1 MB
pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::new();
    let n = nums.len();
    let mut res = vec![-1; n];
    for i in 0..2 * n {
        let j = i % n;
        let x = nums[j];
        while let Some(top) = stack.pop() {
            if nums[top] < x {
                res[top] = x;
            } else {
                stack.push(top);
                break;
            }
        }
        stack.push(j);
    }
    res
}
// stack
#[test]
fn test1_503() {
    assert_eq!(next_greater_elements(vec![1, 2, 1]), vec![2, -1, 2]);
}
