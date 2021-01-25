// https://leetcode-cn.com/problems/binary-subarrays-with-sum/
// Runtime: 0 ms
// Memory Usage: 2.4 MB
pub fn num_subarrays_with_sum(a: Vec<i32>, s: i32) -> i32 {
    let n = a.len();
    let mut count = vec![0; n + 1];
    count[0] = 1;
    let mut sum = 0;
    let mut res = 0;
    for ai in &a {
        sum += ai;
        if sum >= s {
            res += count[(sum - s) as usize];
        }
        count[sum as usize] += 1;
    }
    res
}
// two_pointers hash_table
#[test]
fn test2_930() {
    assert_eq!(num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
}
