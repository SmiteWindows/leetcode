// https://leetcode-cn.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn max_length(arr: Vec<String>) -> i32 {
    let arr = arr
        .into_iter()
        .filter_map(|s| {
            let mut bitset = 0;
            for b in s.bytes() {
                let bit = 1 << (b - b'a');
                if bitset & bit == 0 {
                    bitset |= bit
                } else {
                    return None;
                }
            }
            Some(bitset)
        })
        .collect::<Vec<_>>();
    let n = arr.len();
    let mut res = 0;
    dfs(0, 0, &mut res, &arr, n);
    res as i32
}

fn dfs(start: usize, cur: u32, max: &mut u32, arr: &[u32], n: usize) {
    if start == n {
        *max = (*max).max(cur.count_ones());
    } else {
        if arr[start] & cur == 0 {
            dfs(start + 1, cur | arr[start], max, arr, n);
        }
        dfs(start + 1, cur, max, arr, n);
    }
}
// backtracking bit_manipulation
#[test]
fn test1_1239() {
    use leetcode_prelude::vec_string;
    assert_eq!(max_length(vec_string!["un", "iq", "ue"]), 4);
    assert_eq!(max_length(vec_string!["cha", "r", "act", "ers"]), 6);
    assert_eq!(max_length(vec_string!["abcdefghijklmnopqrstuvwxyz"]), 26);
}
