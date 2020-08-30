// https://leetcode-cn.com/problems/find-all-numbers-disappeared-in-an-array/
// Runtime: 12 ms
// Memory Usage: 2.5 MB
pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut res = Vec::new();
    let n = nums.len();
    for i in 0..n {
        let t = nums[i].abs() as usize - 1;
        if nums[t] > 0 {
            nums[t] = -nums[t];
        }
    }
    for (i, &x) in nums.iter().enumerate() {
        if x > 0 {
            res.push(i as i32 + 1);
        }
    }
    res
}
// array
#[test]
fn test1_448() {
    assert_eq!(
        find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![5, 6]
    );
}
