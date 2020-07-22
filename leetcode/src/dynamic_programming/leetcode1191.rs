// https://leetcode.com/problems/k-concatenation-maximum-sum/
// Runtime: 8 ms
// Memory Usage: 2.9 MB
const MOD: i32 = 1_000_000_007;
pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
    let sum = arr.iter().sum::<i32>();
    let mut prev = 0;
    let mut res = 0;
    let mut k = k as usize;
    let n = arr.len();
    for i in 0..n * k.min(2) {
        prev = arr[i % n].max(prev + arr[i % n]);
        res = res.max(prev);
    }
    while sum > 0 && k > 2 {
        res += sum;
        res %= MOD;
        k -= 1
    }
    res
}
// dynamic_programming
#[test]
fn test1_1191() {
    assert_eq!(k_concatenation_max_sum(vec![1, 2], 3), 9);
    assert_eq!(k_concatenation_max_sum(vec![1, -2, 1], 5), 2);
    assert_eq!(k_concatenation_max_sum(vec![-1, -2], 7), 0);
}
