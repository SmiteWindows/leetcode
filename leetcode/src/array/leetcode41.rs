// https://leetcode-cn.com/problems/first-missing-positive/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    for i in 0..n {
        loop {
            let j = nums[i] - 1;
            if j >= 0 && j < n as i32 && nums[j as usize] != nums[i] {
                nums.swap(i, j as usize);
            } else {
                break;
            }
        }
    }
    for (i, &num) in nums.iter().enumerate().take(n) {
        if num != i as i32 + 1 {
            return (i + 1) as i32;
        }
    }
    (n + 1) as i32
}
// array
#[test]
fn test1_41() {
    assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
    assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
    assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
}
