// https://leetcode.com/problems/design-hashset/
// Runtime: 20 ms
// Memory Usage: 5.7 MB
struct MyHashSet {
    table: Vec<bool>,
}

impl MyHashSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            table: vec![false; 1_000_000],
        }
    }

    fn add(&mut self, key: i32) {
        self.table[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.table[key as usize] = false;
    }

    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        self.table[key as usize]
    }
}
/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */
// design hash_table
#[test]
fn test1_705() {
    let mut hs = MyHashSet::new();
    hs.add(1);
    hs.add(2);
    assert_eq!(hs.contains(1), true);
    assert_eq!(hs.contains(3), false);
    hs.add(2);
    assert_eq!(hs.contains(2), true);
    hs.remove(2);
    assert_eq!(hs.contains(2), false);
}
