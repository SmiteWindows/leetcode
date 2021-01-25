// https://leetcode-cn.com/problems/single-number-iii/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let mut bitmask = 0;
    for i in &nums {
        bitmask ^= i;
    }
    let diff = bitmask & (-bitmask);
    let mut x = 0;
    for i in &nums {
        if i & diff != 0 {
            x ^= i;
        }
    }
    vec![x, bitmask ^ x]
}
// bit_manipulation
#[test]
fn test1_260() {
    assert_eq!(single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
}
