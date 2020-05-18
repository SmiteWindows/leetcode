// https://leetcode.com/problems/design-circular-deque/
// Runtime: 12 ms
// Memory Usage: 2.4 MB
struct MyCircularDeque {
    k: usize,
    start: usize,
    end: usize,
    data: Vec<i32>,
    count: usize,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    /** Initialize your data structure here. Set the size of the deque to be k. */
    fn new(k: i32) -> Self {
        let k = k as usize;
        let data = vec![0; k];
        Self {
            k,
            start: 0,
            end: 0,
            data,
            count: 0,
        }
    }

    /** Adds an item at the front of Deque. Return true if the operation is successful. */
    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.count += 1;
            self.start = (self.start + self.k - 1) % self.k;
            self.data[self.start] = value;
            true
        }
    }

    /** Adds an item at the rear of Deque. Return true if the operation is successful. */
    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.count += 1;
            self.data[self.end] = value;
            self.end = (self.end + 1) % self.k;
            true
        }
    }

    /** Deletes an item from the front of Deque. Return true if the operation is successful. */
    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.count -= 1;
            self.start = (self.start + 1) % self.k;
            true
        }
    }

    /** Deletes an item from the rear of Deque. Return true if the operation is successful. */
    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.count -= 1;
            self.end = (self.end + self.k - 1) % self.k;
            true
        }
    }

    /** Get the front item from the deque. */
    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.start]
        }
    }

    /** Get the last item from the deque. */
    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[(self.end + self.k - 1) % self.k]
        }
    }

    /** Checks whether the circular deque is empty or not. */
    fn is_empty(&self) -> bool {
        self.count == 0
    }

    /** Checks whether the circular deque is full or not. */
    fn is_full(&self) -> bool {
        self.count == self.k
    }
}
/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */
// design queue
#[test]
fn test2_641() {
    let mut queue = MyCircularDeque::new(3);
    assert_eq!(queue.insert_last(1), true);
    assert_eq!(queue.insert_last(2), true);
    assert_eq!(queue.insert_front(3), true);
    assert_eq!(queue.insert_front(4), false);
    assert_eq!(queue.get_rear(), 2);
    assert_eq!(queue.is_full(), true);
    assert_eq!(queue.delete_last(), true);
    assert_eq!(queue.get_rear(), 1);
    assert_eq!(queue.insert_front(4), true);
    assert_eq!(queue.get_front(), 4);
    assert_eq!(queue.delete_front(), true);
    assert_eq!(queue.get_front(), 3);
}
