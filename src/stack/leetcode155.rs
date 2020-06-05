// https://leetcode.com/problems/min-stack/
// Runtime: 4 ms
// Memory Usage: 5.4 MB
struct MinStack {
    nums: Vec<i32>,
    mins: Vec<i32>,
    top: i32,
    min: i32,
}

impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            nums: vec![],
            mins: vec![],
            top: 0,
            min: 0,
        }
    }

    fn push(&mut self, x: i32) {
        self.nums.push(x);
        if let Some(&min) = self.mins.last() {
            self.mins.push(x.min(min))
        } else {
            self.mins.push(x)
        }
        self.min = if let Some(&last) = self.mins.last() {
            last
        } else {
            0
        };
        self.top = if let Some(&last) = self.nums.last() {
            last
        } else {
            0
        };
    }

    fn pop(&mut self) {
        self.nums.pop();
        self.mins.pop();
        self.min = if let Some(&last) = self.mins.last() {
            last
        } else {
            0
        };
        self.top = if let Some(&last) = self.nums.last() {
            last
        } else {
            0
        };
    }

    fn top(&self) -> i32 {
        self.top
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}
/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
// design stack
#[test]
fn test1_155() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(min_stack.get_min(), -3);
    min_stack.pop();
    assert_eq!(min_stack.top(), 0);
    assert_eq!(min_stack.get_min(), -2);
}
