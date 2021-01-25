// https://leetcode-cn.com/problems/design-circular-queue/
// Runtime: 8 ms
// Memory Usage: 2.4 MB
struct MyCircularQueue {
    k: usize,
    start: usize,
    end: usize,
    data: Vec<i32>,
    count: usize,
}

impl MyCircularQueue {
    /** Initialize your data structure here. Set the size of the queue to be k. */
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

    /** Insert an element into the circular queue. Return true if the operation is successful. */
    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.count += 1;
            self.data[self.end] = value;
            self.end = (self.end + 1) % self.k;
            true
        }
    }

    /** Delete an element from the circular queue. Return true if the operation is successful. */
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.count -= 1;
            self.start = (self.start + 1) % self.k;
            true
        }
    }

    /** Get the front item from the queue. */
    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.start]
        }
    }

    /** Get the last item from the queue. */
    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[(self.end + self.k - 1) % self.k]
        }
    }

    /** Checks whether the circular queue is empty or not. */
    fn is_empty(&self) -> bool {
        self.count == 0
    }

    /** Checks whether the circular queue is full or not. */
    fn is_full(&self) -> bool {
        self.count == self.k
    }
}
/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */
// design queue
#[test]
fn test2_622() {
    let mut queue = MyCircularQueue::new(3);
    assert_eq!(queue.en_queue(1), true);
    assert_eq!(queue.en_queue(2), true);
    assert_eq!(queue.en_queue(3), true);
    assert_eq!(queue.en_queue(4), false);
    assert_eq!(queue.rear(), 3);
    assert_eq!(queue.front(), 1);
    assert_eq!(queue.is_full(), true);
    assert_eq!(queue.de_queue(), true);
    assert_eq!(queue.en_queue(4), true);
    assert_eq!(queue.rear(), 4);
    assert_eq!(queue.front(), 2);
}
