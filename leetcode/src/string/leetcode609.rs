// https://leetcode-cn.com/problems/find-duplicate-file-in-system/
// Runtime: 32 ms
// Memory Usage: 6.2 MB
use std::collections::HashMap;
pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
    let mut hm: HashMap<String, Vec<String>> = HashMap::new();
    for s in paths {
        let mut s_iter = s.split_whitespace();
        let dir = s_iter.next().unwrap();
        for f in s_iter {
            let mut f_iter = f.chars();
            let name = f_iter
                .by_ref()
                .take_while(|&c| c != '(')
                .collect::<String>();
            let content = f_iter
                .by_ref()
                .take_while(|&c| c != ')')
                .collect::<String>();
            hm.entry(content)
                .or_default()
                .push(format!("{}/{}", dir, name))
        }
    }
    hm.into_iter()
        .filter_map(|(_, v)| if v.len() > 1 { Some(v) } else { None })
        .collect()
}
// hash_table string
#[test]
fn test2_609() {
    use leetcode_prelude::{assert_eq_sorted, vec2_string, vec_string};
    assert_eq_sorted!(
        find_duplicate(vec_string![
            "root/a 1.txt(abcd) 2.txt(efgh)",
            "root/c 3.txt(abcd)",
            "root/c/d 4.txt(efgh)",
            "root 4.txt(efgh)"
        ]),
        vec2_string![
            ["root/a/2.txt", "root/c/d/4.txt", "root/4.txt"],
            ["root/a/1.txt", "root/c/3.txt"]
        ]
    );
}
