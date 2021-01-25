// https://leetcode-cn.com/problems/total-hamming-distance/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let n = nums.len();
    for i in 0..32 {
        let mut count = 0;
        for &x in &nums {
            if x & 1 << i != 0 {
                count += 1;
            }
        }
        res += count * (n - count);
    }
    res as i32
}
// bit_manipulation
#[test]
fn test1_477() {
    assert_eq!(total_hamming_distance(vec![4, 14, 2]), 6);
}
