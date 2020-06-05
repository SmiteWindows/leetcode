// https://leetcode.com/problems/maximum-frequency-stack/
// Runtime: 48 ms
// Memory Usage: 7.2 MB
use std::collections::HashMap;
#[derive(Default)]
struct FreqStack {
    freq: HashMap<i32, usize>,
    stacks: HashMap<usize, Vec<i32>>,
    max_freq: usize,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, x: i32) {
        let n = self.freq.entry(x).or_default();
        *n += 1;
        self.stacks.entry(*n).or_default().push(x);
        self.max_freq = self.max_freq.max(*n);
    }

    fn pop(&mut self) -> i32 {
        let max_stack: &mut Vec<i32> = self.stacks.get_mut(&self.max_freq).unwrap();
        let x = max_stack.pop().unwrap();
        *self.freq.entry(x).or_default() -= 1;
        if max_stack.is_empty() {
            self.max_freq -= 1;
        }
        x
    }
}
/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 */
// stack hash_table
#[test]
fn test1_895() {
    let mut obj = FreqStack::new();
    obj.push(5);
    obj.push(7);
    obj.push(5);
    obj.push(7);
    obj.push(4);
    obj.push(5);
    assert_eq!(obj.pop(), 5);
    assert_eq!(obj.pop(), 7);
    assert_eq!(obj.pop(), 5);
    assert_eq!(obj.pop(), 4);
}
