// https://leetcode-cn.com/problems/longest-string-chain/
// Runtime: 16 ms
// Memory Usage: 2.1 MB
pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
    let n = words.len();
    let mut v = vec![1; n];
    let mut res = 1;
    words.sort_unstable_by_key(|s| s.len());
    for i in 1..n {
        let cur = &words[i];
        let m = cur.len();
        for j in (0..i).rev() {
            let l = words[j].len();
            if l == m {
                continue;
            } else if l == m - 1 {
                if is_prev(&words[j], &words[i]) {
                    v[i] = v[j] + 1;
                    res = res.max(v[i]);
                    break;
                }
            } else {
                break;
            }
        }
    }
    res
}

fn is_prev(prev: &str, next: &str) -> bool {
    let mut i = 0;
    let mut j = 0;
    while i < prev.len() {
        if prev[i..=i] == next[j..=j] {
            i += 1;
            j += 1;
        } else if i == j {
            j += 1;
        } else {
            return false;
        }
    }
    true
}
// hash_table dynamic_programming
#[test]
fn test1_1048() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        longest_str_chain(vec_string!["a", "b", "ba", "bca", "bda", "bdca"]),
        4
    );
}
