// https://leetcode-cn.com/problems/swap-for-longest-repeated-character-substring/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn max_rep_opt1(text: String) -> i32 {
    let s = text.bytes().collect::<Vec<_>>();
    let n = s.len();
    let mut group = vec![];
    let mut count = vec![0; 26];
    for i in &s {
        count[(i - b'a') as usize] += 1;
    }
    let mut prev = 1;
    for i in 1..n {
        if s[i] == s[i - 1] {
            prev += 1;
        } else {
            group.push((s[i - 1], prev));
            prev = 1;
        }
    }
    group.push((s[n - 1], prev));
    let mut res = 0;
    for g in &group {
        res = res.max(count[(g.0 - b'a') as usize].min(g.1 + 1));
    }
    for i in 1..group.len() - 1 {
        if group[i - 1].0 == group[i + 1].0 && group[i].1 == 1 {
            let b = group[i - 1].0;
            res = res.max(count[(b - b'a') as usize].min(group[i - 1].1 + group[i + 1].1 + 1));
        }
    }
    res
}
// string
#[test]
fn test1_1156() {
    assert_eq!(max_rep_opt1("ababa".to_string()), 3);
    assert_eq!(max_rep_opt1("aaabaaa".to_string()), 6);
    assert_eq!(max_rep_opt1("aaabbaaa".to_string()), 4);
    assert_eq!(max_rep_opt1("aaaaa".to_string()), 5);
    assert_eq!(max_rep_opt1("abcdef".to_string()), 1);
}
