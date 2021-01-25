// https://leetcode-cn.com/problems/subarrays-with-k-different-integers/
// Runtime: 24 ms
// Memory Usage: 2.5 MB
use std::collections::HashMap;
pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
    (at_most(&a, k) - at_most(&a, k - 1)) as i32
}

fn at_most(a: &[i32], mut k: i32) -> usize {
    let n = a.len();
    let mut hm: HashMap<i32, i32> = HashMap::new();
    let mut j = 0;
    let mut res = 0;
    for i in 0..n {
        let count = hm.entry(a[i]).or_default();
        if *count == 0 {
            k -= 1;
        }
        *count += 1;
        while k < 0 {
            let count = hm.entry(a[j]).or_default();
            *count -= 1;
            if *count == 0 {
                k += 1;
            }
            j += 1;
        }
        res += i - j + 1;
    }
    res
}
// sliding_window two_pointers hash_table
#[test]
fn test2_992() {
    assert_eq!(subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2), 7);
    assert_eq!(subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3), 3);
}
