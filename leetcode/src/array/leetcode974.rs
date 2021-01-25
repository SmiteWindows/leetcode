// https://leetcode-cn.com/problems/subarray-sums-divisible-by-k/
// Runtime: 12 ms
// Memory Usage: 2.4 MB
pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
    let n = k as usize;
    let mut count: Vec<i32> = vec![0; n];
    let mut sum = 0;
    let mut res = 0;
    count[0] = 1;
    for x in a {
        sum = (sum + x % k + k) % k;
        res += count[sum as usize];
        count[sum as usize] += 1;
    }
    res
}
// hash_table array
#[test]
fn test2_974() {
    assert_eq!(subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
}
