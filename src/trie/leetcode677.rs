// https://leetcode.com/problems/map-sum-pairs/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::{collections::hash_map::DefaultHasher, collections::HashMap, hash::Hasher};
struct MapSum {
    sum: HashMap<u64, i32>,
    val: HashMap<u64, i32>,
}

impl MapSum {
    /** Initialize your data structure here. */
    fn new() -> Self {
        let sum = HashMap::new();
        let val = HashMap::new();
        Self { sum, val }
    }

    fn insert(&mut self, key: String, val: i32) {
        let mut hasher = DefaultHasher::new();
        for b in key.bytes() {
            hasher.write_u8(b);
            let k = hasher.finish();
            *self.sum.entry(k).or_default() += val;
        }
        let k = hasher.finish();
        if let Some(prev) = self.val.insert(k, val) {
            let mut hasher = DefaultHasher::new();
            for b in key.bytes() {
                hasher.write_u8(b);
                let k = hasher.finish();
                *self.sum.entry(k).or_default() -= prev;
            }
        }
    }

    fn sum(&mut self, prefix: String) -> i32 {
        let mut hasher = DefaultHasher::new();
        for b in prefix.bytes() {
            hasher.write_u8(b);
        }
        let k = hasher.finish();
        *self.sum.entry(k).or_default()
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
