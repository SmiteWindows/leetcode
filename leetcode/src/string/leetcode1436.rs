// https://leetcode-cn.com/problems/destination-city/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut citys: HashSet<&str> = HashSet::new();
    for val in paths.iter() {
        citys.insert(&val[0]);
    }
    for val in paths.iter() {
        if citys.insert(&val[1]) {
            return val[1].to_string();
        };
    }
    "".to_string()
}
// string
#[test]
fn test1_1436() {
    use leetcode_prelude::vec2_string;
    assert_eq!(
        dest_city(vec2_string![
            ["London", "New York"],
            ["New York", "Lima"],
            ["Lima", "Sao Paulo"]
        ]),
        String::from("Sao Paulo")
    );
    assert_eq!(
        dest_city(vec2_string![["B", "C"], ["D", "B"], ["C", "A"]]),
        String::from("A")
    );
    assert_eq!(dest_city(vec2_string![["A", "Z"]]), String::from("Z"));
}
