// https://leetcode.com/problems/group-anagrams/
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
    res.sort_by(|a, b| a.len().cmp(&b.len()));
    res
}
// hash_table string
#[test]
fn test1_49() {
    assert_eq!(
        group_anagrams(vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat")
        ]),
        vec![
            vec![String::from("bat")],
            vec![String::from("nat"), String::from("tan")],
            vec![
                String::from("ate"),
                String::from("eat"),
                String::from("tea")
            ]
        ]
    );
}
