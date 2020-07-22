// https://leetcode.com/problems/find-duplicate-file-in-system/
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
    let mut a = find_duplicate(vec![
        String::from("root/a 1.txt(abcd) 2.txt(efgh)"),
        String::from("root/c 3.txt(abcd)"),
        String::from("root/c/d 4.txt(efgh)"),
        String::from("root 4.txt(efgh)"),
    ]);
    a.sort();
    assert_eq!(
        a,
        vec![
            vec![String::from("root/a/1.txt"), String::from("root/c/3.txt")],
            vec![
                String::from("root/a/2.txt"),
                String::from("root/c/d/4.txt"),
                String::from("root/4.txt")
            ],
        ]
    );
}
