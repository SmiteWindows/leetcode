// https://leetcode.com/problems/find-all-anagrams-in-a-string/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut res = vec![];
    let mut ss = vec![0_usize; 26];
    let mut sp = vec![0_usize; 26];
    let s = s.bytes().collect::<Vec<_>>();
    let p = p.bytes().collect::<Vec<_>>();
    if s.len() < p.len() {
        return res;
    }
    for i in 0..p.len() {
        let c = s[i] as usize - 'a' as usize;
        ss[c] += 1;
    }
    for i in 0..p.len() {
        let c = p[i] as usize - 'a' as usize;
        sp[c] += 1;
    }
    if ss == sp {
        res.push(0);
    }
    for i in 1..=(s.len() - p.len()) {
        let c = s[i - 1] as usize - 'a' as usize;
        let d = s[..][i + p.len() - 1] as usize - 'a' as usize;
        ss[c] -= 1;
        ss[d] += 1;
        if ss == sp {
            res.push(i as i32);
        }
    }
    res
}
// hash_table
#[test]
fn test1_438() {
    assert_eq!(
        find_anagrams(String::from("cbaebabacd"), String::from("abc")),
        vec![0, 6]
    );
    assert_eq!(
        find_anagrams(String::from("abab"), String::from("ab")),
        vec![0, 1, 2]
    );
}
