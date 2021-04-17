// https://leetcode-cn.com/problems/insert-delete-getrandom-o1/
// Runtime: 8 ms
// Memory Usage: 5.9 MB
use rand::prelude::*;
use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};
struct RandomizedSet {
    rng: ThreadRng,
    indexes: HashMap<i32, usize>,
    values: Vec<i32>,
}

impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            rng: rand::thread_rng(), // miri test 有误，注意修正
            indexes: HashMap::new(),
            values: Vec::new(),
        }
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        match self.indexes.entry(val) {
            Occupied(_) => false,
            Vacant(e) => {
                e.insert(self.values.len());
                self.values.push(val);
                true
            }
        }
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if let Some(index) = self.indexes.remove(&val) {
            let last_index = self.values.len() - 1;
            let last_value = self.values[last_index];
            if index != last_index {
                self.values.swap(index, last_index);
                let old_index = self.indexes.get_mut(&last_value).unwrap();
                *old_index = index;
            }
            self.values.pop();
            true
        } else {
            false
        }
    }

    /** Get a random element from the set. */
    fn get_random(&mut self) -> i32 {
        let index = self.rng.gen_range(0..self.values.len());
        self.values[index]
    }
}
/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
// design array hash_table
#[test]
#[cfg_attr(miri, ignore)]
fn test2_380() {
    let mut obj = RandomizedSet::new();
    assert_eq!(obj.insert(1), true);
    assert_eq!(obj.remove(2), false);
    assert_eq!(obj.insert(2), true);
    // assert_eq!(obj.get_random(), 1);
    // assert_eq!(obj.get_random(), 2);
    assert_eq!(obj.remove(1), true);
    assert_eq!(obj.insert(2), false);
    assert_eq!(obj.get_random(), 2);
}
