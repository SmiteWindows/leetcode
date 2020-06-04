// https://leetcode.com/problems/range-sum-query-immutable/
// Runtime: 4 ms
// Memory Usage: 7.5 MB
struct NumArray {
    prefix_sums: Vec<i32>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut nums = nums;
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        Self { prefix_sums: nums }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let i = i as usize;
        let j = j as usize;
        if i == 0 {
            self.prefix_sums[j]
        } else {
            self.prefix_sums[j] - self.prefix_sums[i - 1]
        }
    }
}
/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(i, j);
 */
// dynamic_programming
#[test]
fn test1_303() {
    let obj = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(obj.sum_range(0, 2), 1);
    assert_eq!(obj.sum_range(2, 5), -1);
    assert_eq!(obj.sum_range(0, 5), -3);
}
