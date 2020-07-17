// https://leetcode.com/problems/design-skiplist/
// Runtime: 28 ms
// Memory Usage: 8.7 MB
use std::collections::HashMap;
struct Skiplist {
    data: HashMap<i32, usize>,
}

impl Skiplist {
    fn new() -> Self {
        let data = HashMap::new();
        Self { data }
    }

    fn search(&self, target: i32) -> bool {
        self.data.contains_key(&target)
    }

    fn add(&mut self, num: i32) {
        *self.data.entry(num).or_default() += 1;
    }

    fn erase(&mut self, num: i32) -> bool {
        if !self.data.contains_key(&num) {
            return false;
        }
        let count = self.data.get_mut(&num).unwrap();
        *count -= 1;
        if *count == 0 {
            self.data.remove(&num);
        }
        true
    }
}
/**
 * Your Skiplist object will be instantiated and called as such:
 * let obj = Skiplist::new();
 * let ret_1: bool = obj.search(target);
 * obj.add(num);
 * let ret_3: bool = obj.erase(num);
 */
// design
#[test]
fn test1_1206() {
    let mut obj = Skiplist::new();
    obj.add(1);
    obj.add(2);
    obj.add(3);
    assert_eq!(obj.search(0), false);
    obj.add(4);
    assert_eq!(obj.search(1), true);
    assert_eq!(obj.erase(0), false);
    assert_eq!(obj.erase(1), true);
    assert_eq!(obj.search(1), false);
}
