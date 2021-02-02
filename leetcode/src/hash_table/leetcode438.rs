// https://leetcode-cn.com/problems/find-all-anagrams-in-a-string/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut res = Vec::new();
    let mut ss = vec![0_usize; 26];
    let mut sp = vec![0_usize; 26];
    let s = s.bytes().collect::<Vec<_>>();
    let p = p.bytes().collect::<Vec<_>>();
    let sn = s.len();
    let pn = p.len();
    if sn < pn {
        return res;
    }
    for &si in s.iter().take(pn) {
        let c = si as usize - 'a' as usize;
        ss[c] += 1;
    }
    for &pi in &p {
        let c = pi as usize - 'a' as usize;
        sp[c] += 1;
    }
    if ss == sp {
        res.push(0);
    }
    for i in 1..=(sn - pn) {
        let c = s[i - 1] as usize - 'a' as usize;
        let d = s[..][i + pn - 1] as usize - 'a' as usize;
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
        find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
        vec![0, 6]
    );
    assert_eq!(
        find_anagrams("abab".to_string(), "ab".to_string()),
        vec![0, 1, 2]
    );
}
