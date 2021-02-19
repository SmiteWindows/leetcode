// https://leetcode-cn.com/problems/implement-stack-using-queues/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::VecDeque;
struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        self.queue.pop_back().unwrap()
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        *self.queue.back().unwrap()
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}
/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
// design stack
#[test]
fn test1_225() {
    let mut stack = MyStack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.top(), 2);
    assert_eq!(stack.pop(), 2);
    assert_eq!(stack.empty(), false);
}
