// https://leetcode-cn.com/problems/maximum-number-of-non-overlapping-substrings/
#[allow(clippy::many_single_char_names)]
// Runtime: 16 ms
// Memory Usage: 2.3 MB
pub fn max_num_of_substrings(s: String) -> Vec<String> {
    let n = s.len();
    let s = s.bytes().collect::<Vec<u8>>();
    let mut l = vec![usize::MAX; 26];
    let mut r = vec![usize::MIN; 26];
    for (i, si) in s.iter().enumerate().take(n) {
        let j = (si - b'a') as usize;
        l[j] = l[j].min(i);
        r[j] = r[j].max(i);
    }
    let mut res = Vec::new();
    let mut end = 0;
    for i in 0..n {
        let j = (s[i] - b'a') as usize;
        if i == l[j] {
            if let Some(new_end) = check(i, &l, &r, &s) {
                if new_end < end {
                    res.pop();
                }
                res.push(s[i..new_end].iter().map(|&b| b as char).collect::<String>());
                end = new_end;
            }
        }
    }
    res
}

fn check(start: usize, l: &[usize], r: &[usize], s: &[u8]) -> Option<usize> {
    let mut end = r[(s[start] - b'a') as usize] + 1;
    let mut i = start;
    while i < end {
        if l[(s[i] - b'a') as usize] < start {
            return None;
        } else {
            end = end.max(r[(s[i] - b'a') as usize] + 1);
        }
        i += 1;
    }
    Some(end)
}
// greedy
#[test]
fn test1_1520() {
    assert_eq!(
        max_num_of_substrings("adefaddaccc".to_string()),
        vec!["e".to_string(), "f".to_string(), "ccc".to_string()]
    );
    assert_eq!(
        max_num_of_substrings("abbaccd".to_string()),
        vec!["bb".to_string(), "cc".to_string(), "d".to_string()]
    );
}
