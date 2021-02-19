// https://leetcode-cn.com/problems/check-if-string-is-transformable-with-substring-sort-operations/
// Runtime: 8 ms
// Memory Usage: 3.5 MB
pub fn is_transformable(s: String, t: String) -> bool {
    let n = s.len();
    let s: Vec<u8> = s.bytes().collect();
    let t: Vec<u8> = t.bytes().collect();
    let mut idx: Vec<Vec<usize>> = vec![Vec::new(); 10];
    for i in (0..n).rev() {
        idx[(s[i] - b'0') as usize].push(i);
    }
    for &ti in t.iter().take(n) {
        let k = (ti - b'0') as usize;
        if idx[k].is_empty() {
            return false;
        }
        let p = idx[k].pop().unwrap();
        for i in idx.iter().take(k) {
            if let Some(&q) = i.last() {
                if q < p {
                    return false;
                }
            }
        }
    }
    true
}
// string greedy
#[test]
fn test1_1585() {
    assert_eq!(
        is_transformable("84532".to_string(), "34852".to_string()),
        true
    );
    assert_eq!(
        is_transformable("34521".to_string(), "23415".to_string()),
        true
    );
    assert_eq!(
        is_transformable("12345".to_string(), "12435".to_string()),
        false
    );
    assert_eq!(is_transformable("1".to_string(), "2".to_string()), false);
}
