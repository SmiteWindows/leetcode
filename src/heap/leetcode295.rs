// https://leetcode.com/problems/find-median-from-data-stream/
// Runtime: 28 ms
// Memory Usage: 11 MB
use std::{cmp::Reverse, collections::BinaryHeap};
#[derive(Default)]
struct MedianFinder {
    lo: BinaryHeap<i32>,
    hi: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self::default()
    }

    fn add_num(&mut self, num: i32) {
        self.hi.push(Reverse(num));
        let smallest = self.hi.pop().unwrap().0;
        self.lo.push(smallest);
        if self.lo.len() > self.hi.len() + 1 {
            self.hi.push(Reverse(self.lo.pop().unwrap()));
        }
    }

    fn find_median(&self) -> f64 {
        if (self.hi.len() + self.lo.len()) % 2 == 0 {
            (self.hi.peek().unwrap().0 + *self.lo.peek().unwrap()) as f64 / 2.0
        } else {
            *self.lo.peek().unwrap() as f64
        }
    }
}
/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
// design heap
#[test]
fn test1_295() {
    let mut obj = MedianFinder::new();
    obj.add_num(1);
    obj.add_num(2);
    assert!((obj.find_median() - 1.5) <= f64::EPSILON);
    obj.add_num(3);
    assert!((obj.find_median() - 2.0) <= f64::EPSILON);
}
