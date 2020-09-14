// https://leetcode-cn.com/problems/data-stream-as-disjoint-intervals/
// Runtime: 20 ms
// Memory Usage: 7 MB
use std::collections::{BTreeMap, HashSet};

struct SummaryRanges {
    data: BTreeMap<i32, i32>,
    seen: HashSet<i32>,
}

impl SummaryRanges {
    /** Initialize your data structure here. */
    fn new() -> Self {
        let data = BTreeMap::new();
        let seen = HashSet::new();
        Self { data, seen }
    }

    fn add_num(&mut self, val: i32) {
        if !self.seen.insert(val) {
            return;
        }
        let mut l = val;
        let mut r = val;
        if let Some(&right) = self.data.get(&(val + 1)) {
            r = right;
        }
        if let Some((&left, &limit)) = self.data.range(..val).rev().next() {
            if limit == val - 1 {
                l = left;
            }
        }
        if l < val {
            self.data.remove(&l);
        }
        if r > val {
            self.data.remove(&(val + 1));
        }
        self.data.insert(l, r);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.data.iter().map(|(&k, &v)| vec![k, v]).collect()
    }
}
/**
 * Your SummaryRanges object will be instantiated and called as such:
 * let obj = SummaryRanges::new();
 * obj.add_num(val);
 * let ret_2: Vec<Vec<i32>> = obj.get_intervals();
 */
// binary_search ordered_map
#[test]
fn test1_352() {
    use leetcode_prelude::vec2;
    let mut obj = SummaryRanges::new();
    obj.add_num(1);
    assert_eq!(obj.get_intervals(), vec2![[1, 1]]);
    obj.add_num(3);
    assert_eq!(obj.get_intervals(), vec2![[1, 1], [3, 3]]);
    obj.add_num(7);
    assert_eq!(obj.get_intervals(), vec2![[1, 1], [3, 3], [7, 7]]);
    obj.add_num(2);
    assert_eq!(obj.get_intervals(), vec2![[1, 3], [7, 7]]);
    obj.add_num(6);
    assert_eq!(obj.get_intervals(), vec2![[1, 3], [6, 7]]);
}
