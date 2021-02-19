// https://leetcode-cn.com/problems/making-file-names-unique/
// Runtime: 68 ms
// Memory Usage: 10.6 MB
use std::collections::{HashMap, HashSet};
pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
    let mut hs = HashSet::new();
    let mut hm = HashMap::new();
    let mut res = Vec::new();
    'outer: for name in names {
        if !hs.insert(name.to_string()) {
            let mut i = *hm.get(&name).unwrap_or(&1);
            loop {
                let new_name = format!("{}({})", name, i);
                if hs.insert(new_name.to_string()) {
                    res.push(new_name);
                    hm.insert(name, i + 1);
                    continue 'outer;
                }
                i += 1;
            }
        } else {
            res.push(name);
        }
    }
    res
}
// hast_table string
#[test]
fn test2_1487() {
    assert_eq!(
        get_folder_names(vec![
            "pes".to_string(),
            "fifa".to_string(),
            "gta".to_string(),
            "pes(2019)".to_string()
        ]),
        vec![
            "pes".to_string(),
            "fifa".to_string(),
            "gta".to_string(),
            "pes(2019)".to_string()
        ]
    );
    assert_eq!(
        get_folder_names(vec![
            "gta".to_string(),
            "gta(1)".to_string(),
            "gta".to_string(),
            "avalon".to_string()
        ]),
        vec![
            "gta".to_string(),
            "gta(1)".to_string(),
            "gta(2)".to_string(),
            "avalon".to_string()
        ]
    );
    assert_eq!(
        get_folder_names(vec![
            "onepiece".to_string(),
            "onepiece(1)".to_string(),
            "onepiece(2)".to_string(),
            "onepiece(3)".to_string(),
            "onepiece".to_string()
        ]),
        vec![
            "onepiece".to_string(),
            "onepiece(1)".to_string(),
            "onepiece(2)".to_string(),
            "onepiece(3)".to_string(),
            "onepiece(4)".to_string()
        ]
    );
    assert_eq!(
        get_folder_names(vec![
            "wano".to_string(),
            "wano".to_string(),
            "wano".to_string(),
            "wano".to_string()
        ]),
        vec![
            "wano".to_string(),
            "wano(1)".to_string(),
            "wano(2)".to_string(),
            "wano(3)".to_string()
        ]
    );
    assert_eq!(
        get_folder_names(vec![
            "kaido".to_string(),
            "kaido(1)".to_string(),
            "kaido".to_string(),
            "kaido(1)".to_string()
        ]),
        vec![
            "kaido".to_string(),
            "kaido(1)".to_string(),
            "kaido(2)".to_string(),
            "kaido(1)(1)".to_string()
        ]
    );
}
