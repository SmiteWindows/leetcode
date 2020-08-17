// https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/
// Runtime: 12 ms
// Memory Usage: 6 MB
use rand::prelude::{thread_rng, Rng, ThreadRng};
use std::collections::{BinaryHeap, HashMap};
struct RandomizedCollection {
    rng: ThreadRng,
    indexes: HashMap<i32, BinaryHeap<usize>>,
    choices: Vec<i32>,
}

impl RandomizedCollection {
    /** Initialize your data structure here. */
    fn new() -> Self {
        let rng = thread_rng();
        let indexes = HashMap::new();
        let choices = vec![];
        Self {
            rng,
            indexes,
            choices,
        }
    }

    /** Inserts a value to the collection. Returns true if the collection did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        let ids = self.indexes.entry(val).or_default();
        ids.push(self.choices.len());
        self.choices.push(val);
        ids.len() == 1
    }

    /** Removes a value from the collection. Returns true if the collection contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if let Some(ids) = self.indexes.get_mut(&val) {
            if let Some(index) = ids.pop() {
                let last_index = self.choices.len() - 1;
                let last_value = self.choices[last_index];
                if last_value != val {
                    let other_ids = self.indexes.get_mut(&last_value).unwrap();
                    other_ids.pop();
                    other_ids.push(index);
                    self.choices.swap(index, last_index);
                }
                self.choices.pop();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /** Get a random element from the collection. */
    fn get_random(&mut self) -> i32 {
        self.choices[self.rng.gen_range(0, self.choices.len())]
    }
}
/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * let obj = RandomizedCollection::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
// design array hash_table
#[test]
fn test1_381() {
    let mut obj = RandomizedCollection::new();
    assert_eq!(obj.insert(1), true);
    assert_eq!(obj.insert(1), false);
    assert_eq!(obj.insert(2), true);
    // assert_eq!(obj.get_random(), 2);
    assert_eq!(obj.remove(1), true);
    // assert_eq!(obj.get_random(), 2);
}
