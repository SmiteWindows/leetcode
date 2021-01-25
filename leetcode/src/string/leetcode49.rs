// https://leetcode-cn.com/problems/group-anagrams/
// Runtime: 8 ms
// Memory Usage: 4.3 MB
use std::collections::HashMap;
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hm: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    for s in strs {
        let mut v = s.chars().collect::<Vec<_>>();
        v.sort_unstable();
        hm.entry(v).or_default().push(s);
    }
    let mut res = vec![];
    for (_, mut v) in hm {
        v.sort_unstable();
        res.push(v.to_vec());
    }
    res.sort_by_key(|a| a.len());
    res
}
// hash_table string
#[test]
fn test2_49() {
    use leetcode_prelude::{vec2_string, vec_string};
    assert_eq!(
        group_anagrams(vec_string!["eat", "tea", "tan", "ate", "nat", "bat"]),
        vec2_string![["bat"], ["nat", "tan"], ["ate", "eat", "tea"]]
    );
}
