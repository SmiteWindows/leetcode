// https://leetcode-cn.com/problems/single-number/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut a = 0;
    for i in nums {
        a ^= i;
    }
    a
}
// hash_table bit_manipulation
#[test]
fn test1_136() {
    assert_eq!(single_number(vec![2, 2, 1]), 1);
    assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
}
