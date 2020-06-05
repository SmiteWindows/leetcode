// https://leetcode.com/problems/lru-cache/
struct LRUCache {
    hash_map: HashMap<i32, Element>,
    list: LinkedList<Element>,
    length: usize,
    capacity: usize,
}
use std::collections::{HashMap, LinkedList};
#[derive(Debug, Clone, Copy)]
struct Element {
    key: i32,
    value: i32,
}

impl Element {
    fn new(key: i32, value: i32) -> Self {
        Self { key, value }
    }
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        let hash_map = HashMap::with_capacity(capacity);
        let list = LinkedList::new();
        let length = 0;
        Self {
            hash_map,
            list,
            length,
            capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&element) = self.hash_map.get(&key) {
            self.list.push_front(element);
            element.value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.put_element(key, value).is_none() {
            self.length += 1;
        }
        if self.length > self.capacity {
            self.pop_element();
            self.length -= 1;
        }
    }

    fn put_element(&mut self, key: i32, value: i32) -> Option<Element> {
        let new_element = Element::new(key, value);
        let old_element = self.hash_map.insert(key, new_element);
        self.list.push_front(new_element);
        old_element
    }
    fn pop_element(&mut self) -> Option<Element> {
        if let Some(element) = self.list.pop_back() {
            self.hash_map.remove(&element.key);
            Some(element)
        } else {
            None
        }
    }
}
/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
#[test]
#[ignore]
fn test() {
    let mut obj = LRUCache::new(2);
    assert_eq!(obj.get(2), -1);
    obj.put(2, 6);
    assert_eq!(obj.get(1), -1);
    obj.put(1, 5);
    obj.put(1, 2);
    assert_eq!(obj.get(1), 2);
    assert_eq!(obj.get(2), 6);

    let mut obj = LRUCache::new(3);
    obj.put(1, 1);
    obj.put(2, 2);
    obj.put(3, 3);
    obj.put(4, 4);
    assert_eq!(obj.get(4), 4);
    assert_eq!(obj.get(3), 3);
    assert_eq!(obj.get(2), 2);
    assert_eq!(obj.get(1), -1);
    obj.put(5, 5);
    assert_eq!(obj.get(1), -1);
    assert_eq!(obj.get(2), 2);
    assert_eq!(obj.get(3), 3);
    assert_eq!(obj.get(4), -1);
    assert_eq!(obj.get(5), 5);
}
