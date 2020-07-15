// https://leetcode.com/problems/all-oone-data-structure/
// Runtime: 8 ms
// Memory Usage: 9.9 MB
use std::collections::HashMap;
struct AllOne {
    dict: HashMap<String, usize>,
}

impl AllOne {
    /** Initialize your data structure here. */
    fn new() -> Self {
        let dict = HashMap::new();
        Self { dict }
    }

    /** Inserts a new key <Key> with value 1. Or increments an existing key by 1. */
    fn inc(&mut self, key: String) {
        self.dict.entry(key).and_modify(|v| *v += 1).or_insert(1);
    }

    /** Decrements an existing key by 1. If Key's value is 1, remove it from the data structure. */
    fn dec(&mut self, key: String) {
        if !self.dict.contains_key(&key) {
            return;
        }
        if self.dict[&key] == 1 {
            self.dict.remove(&key);
        } else {
            *self.dict.get_mut(&key).unwrap() -= 1;
        }
    }

    /** Returns one of the keys with maximal value. */
    fn get_max_key(&mut self) -> String {
        self.dict
            .iter()
            .max_by_key(|(_, &v)| v)
            .unwrap_or((&String::from(""), &0))
            .0
            .to_string()
    }

    /** Returns one of the keys with Minimal value. */
    fn get_min_key(&mut self) -> String {
        self.dict
            .iter()
            .min_by_key(|(_, &v)| v)
            .unwrap_or((&String::from(""), &0))
            .0
            .to_string()
    }
}
/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */
// design
#[test]
fn test1_432() {
    let mut obj = AllOne::new();
    obj.inc("abc".to_string());
    obj.inc("bcd".to_string());
    obj.inc("bcd".to_string());
    assert_eq!(obj.get_max_key(), "bcd".to_string());
    obj.dec("abc".to_string());
    assert_eq!(obj.get_max_key(), "bcd".to_string());
    assert_eq!(obj.get_min_key(), "bcd".to_string());
}
