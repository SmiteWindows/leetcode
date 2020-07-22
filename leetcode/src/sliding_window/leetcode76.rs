// https://leetcode.com/problems/minimum-window-substring/
#![allow(clippy::many_single_char_names)]
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn min_window(s: String, t: String) -> String {
    let mut freq = vec![0_usize; 256];
    let mut limit = vec![0_usize; 256];
    let mut k = 0;
    let n = s.len();
    for b in t.bytes() {
        if limit[b as usize] == 0 {
            k += 1;
        }
        limit[b as usize] += 1;
    }
    let v = s.bytes().collect::<Vec<_>>();
    let mut start = 0;
    let mut end = 0;
    let mut res = (usize::MAX, "");
    loop {
        if k == 0 {
            if end - start < res.0 {
                res = (end - start, &s[start..end]);
            }
            if limit[v[start] as usize] != 0 {
                if freq[v[start] as usize] == limit[v[start] as usize] {
                    k += 1;
                }
                freq[v[start] as usize] -= 1;
            }
            start += 1;
        } else if end == n {
            break;
        } else {
            if limit[v[end] as usize] != 0 {
                freq[v[end] as usize] += 1;
                if freq[v[end] as usize] == limit[v[end] as usize] {
                    k -= 1;
                }
            }
            end += 1;
        }
    }
    res.1.to_string()
}
// hash_table two_pointers string sliding_window
#[test]
fn test2_76() {
    assert_eq!(
        min_window(String::from("ADOBECODEBANC"), String::from("ABC")),
        String::from("BANC")
    );
}
