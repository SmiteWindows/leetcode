// https://leetcode.com/problems/set-mismatch/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut arr = vec![0; nums.len() + 1];
    let mut dup = 0;
    let mut missing = 1;
    for &num in &nums {
        arr[num as usize] += 1;
    }
    for (i, &a) in arr.iter().enumerate() {
        if a == 0 {
            missing = i;
        } else if a == 2 {
            dup = i;
        }
    }
    vec![dup as i32, missing as i32]
}
// math hash_table
#[test]
fn test1_645() {
    assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
}
