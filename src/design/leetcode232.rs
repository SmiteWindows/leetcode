// https://leetcode.com/problems/implement-queue-using-stacks/
// Runtime: 0 ms
// Memory Usage: 2 MB
struct MyQueue {
    stack: Vec<i32>,
    temp: Vec<i32>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            temp: Vec::new(),
        }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        while let Some(x) = self.stack.pop() {
            self.temp.push(x);
        }
        let res = self.temp.pop().unwrap();
        while let Some(x) = self.temp.pop() {
            self.stack.push(x)
        }
        res
    }

    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        while let Some(x) = self.stack.pop() {
            self.temp.push(x);
        }
        let res = self.temp.pop().unwrap();
        self.stack.push(res);
        while let Some(x) = self.temp.pop() {
            self.stack.push(x)
        }
        res
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}
/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
// design stack
#[test]
fn test2_232() {
    let mut queue = MyQueue::new();
    queue.push(1);
    queue.push(2);
    assert_eq!(queue.peek(), 1);
    assert_eq!(queue.pop(), 1);
    assert_eq!(queue.empty(), false);
}
