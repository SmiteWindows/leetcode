// https://leetcode.com/problems/design-a-stack-with-increment-operation/
// Runtime: 4 ms
// Memory Usage: 2.9 MB
struct CustomStack {
    stack: Vec<i32>,
    increment: Vec<i32>,
    n: usize,
    max_size: usize,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    fn new(max_size: i32) -> Self {
        let max_size = max_size as usize;
        let increment = vec![0; 1_usize + max_size];
        Self {
            stack: vec![],
            increment,
            n: 0,
            max_size,
        }
    }

    fn push(&mut self, x: i32) {
        if self.n != self.max_size {
            self.stack.push(x);
            self.n += 1;
        }
    }

    fn pop(&mut self) -> i32 {
        if let Some(mut top) = self.stack.pop() {
            self.increment[self.n - 1] += self.increment[self.n];
            top += self.increment[self.n];
            self.increment[self.n] = 0;
            self.n -= 1;
            top
        } else {
            -1
        }
    }

    fn increment(&mut self, k: i32, val: i32) {
        let k = k as usize;
        if k > self.n {
            self.increment[self.n] += val;
        } else {
            self.increment[k] += val;
        }
    }
}
/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */
// design stack
#[test]
fn test2_1381() {
    let mut stack = CustomStack::new(3);
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.pop(), 2);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.increment(5, 100);
    stack.increment(2, 100);
    assert_eq!(stack.pop(), 103);
    assert_eq!(stack.pop(), 202);
    assert_eq!(stack.pop(), 201);
    assert_eq!(stack.pop(), -1);
}
