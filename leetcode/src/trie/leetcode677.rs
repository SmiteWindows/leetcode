// https://leetcode-cn.com/problems/map-sum-pairs/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
struct MapSum {
    map: HashMap<String, i32>,
}

impl MapSum {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        self.map.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        self.map
            .keys()
            .filter(|&s| s.starts_with(&prefix))
            .map(|s| self.map[s])
            .sum()
    }
}
/**
 * Your MapSum object will be instantiated and called as such:
 * let obj = MapSum::new();
 * obj.insert(key, val);
 * let ret_2: i32 = obj.sum(prefix);
 */
// trie
#[test]
fn test1_677() {
    let mut obj = MapSum::new();
    obj.insert("apple".to_string(), 3);
    assert_eq!(obj.sum("ap".to_string()), 3);
    obj.insert("app".to_string(), 2);
    assert_eq!(obj.sum("ap".to_string()), 5);
}
