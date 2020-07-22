// https://leetcode.com/problems/design-hashmap/
// Runtime: 24 ms
// Memory Usage: 8.6 MB
struct MyHashMap {
    table: Vec<i32>,
}

impl MyHashMap {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            table: vec![-1; 1_000_000],
        }
    }

    /** value will always be non-negative. */
    fn put(&mut self, key: i32, value: i32) {
        self.table[key as usize] = value;
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&self, key: i32) -> i32 {
        self.table[key as usize]
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: i32) {
        self.table[key as usize] = -1;
    }
}
/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */
// design hash_table
#[test]
fn test1_706() {
    let mut hm = MyHashMap::new();
    hm.put(1, 1);
    hm.put(2, 2);
    assert_eq!(hm.get(1), 1);
    assert_eq!(hm.get(3), -1);
    hm.put(2, 1);
    assert_eq!(hm.get(2), 1);
    hm.remove(2);
    assert_eq!(hm.get(2), -1);
}
