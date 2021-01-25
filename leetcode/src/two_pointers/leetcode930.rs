// https://leetcode-cn.com/problems/binary-subarrays-with-sum/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn num_subarrays_with_sum(a: Vec<i32>, s: i32) -> i32 {
    let mut i_lo = 0;
    let mut i_hi = 0;
    let mut sum_lo = 0;
    let mut sum_hi = 0;
    let mut ans = 0;
    for (j, aj) in a.iter().enumerate() {
        // While sum_lo is too big, i_lo++
        sum_lo += aj;
        while i_lo < j && sum_lo > s {
            sum_lo -= a[i_lo];
            i_lo += 1;
        }
        // While sum_hi is too big, or equal and we can move, i_hi++
        sum_hi += aj;
        while i_hi < j && (sum_hi > s || sum_hi == s && a[i_hi] == 0) {
            sum_hi -= a[i_hi];
            i_hi += 1;
        }
        if sum_lo == s {
            ans += i_hi - i_lo + 1;
        }
    }
    ans as i32
}
// two_pointers hash_table
#[test]
fn test1_930() {
    assert_eq!(num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
}
