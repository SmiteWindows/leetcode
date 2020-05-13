// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut res = Vec::new();
    for i in 0..nums.len() {
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
