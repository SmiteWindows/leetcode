// https://leetcode-cn.com/problems/isomorphic-strings/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut hmst = HashMap::new();
    let mut hmts = HashMap::new();
    let mut is = s.chars();
    let mut it = t.chars();
    while let (Some(cs), Some(ct)) = (is.next(), it.next()) {
        if let Some(&vt) = hmst.get(&cs) {
            if vt != ct {
                return false;
            }
        } else {
            hmst.insert(cs, ct);
        }
        if let Some(&vs) = hmts.get(&ct) {
            if vs != cs {
                return false;
            }
        } else {
            hmts.insert(ct, cs);
        }
    }
    true
}
// hash_table
#[test]
fn test1_205() {
    assert_eq!(
        is_isomorphic(String::from("egg"), String::from("add")),
        true
    );
    assert_eq!(
        is_isomorphic(String::from("foo"), String::from("bar")),
        false
    );
    assert_eq!(
        is_isomorphic(String::from("paper"), String::from("title")),
        true
    );
}
