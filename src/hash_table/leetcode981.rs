// https://leetcode.com/problems/time-based-key-value-store/
// Runtime: 100 ms
// Memory Usage: 85.5 MB
use std::collections::HashMap;
#[derive(Default)]
struct TimeMap {
    map: HashMap<String, Vec<(String, i32)>>,
}

impl TimeMap {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((value, timestamp));
    }

    fn get(&mut self, key: String, timestamp: i32) -> String {
        let values = self.map.entry(key).or_default();
        match values.binary_search_by_key(&timestamp, |v| v.1) {
            Ok(i) => values[i].0.to_string(),
            Err(i) => {
                if i > 0 {
                    values[i - 1].0.to_string()
                } else {
                    "".to_string()
                }
            }
        }
    }
}
/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
// binary_search hash_table
#[test]
fn test2_981() {
    let mut kv = TimeMap::new();
    kv.set("foo".to_string(), "bar".to_string(), 1);
    assert_eq!(kv.get("foo".to_string(), 1), "bar".to_string());
    assert_eq!(kv.get("foo".to_string(), 3), "bar".to_string());
    kv.set("foo".to_string(), "bar2".to_string(), 4);
    assert_eq!(kv.get("foo".to_string(), 4), "bar2".to_string());
    assert_eq!(kv.get("foo".to_string(), 5), "bar2".to_string());
    let mut kv = TimeMap::new();
    kv.set("love".to_string(), "high".to_string(), 10);
    kv.set("love".to_string(), "low".to_string(), 20);
    assert_eq!(kv.get("love".to_string(), 5), "".to_string());
    assert_eq!(kv.get("love".to_string(), 10), "high".to_string());
    assert_eq!(kv.get("love".to_string(), 15), "high".to_string());
    assert_eq!(kv.get("love".to_string(), 20), "low".to_string());
    assert_eq!(kv.get("love".to_string(), 25), "low".to_string());
}
