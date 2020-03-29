// https://leetcode.com/problems/design-circular-queue/
struct MyCircularQueue {}

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
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    /** Initialize your data structure here. Set the size of the queue to be k. */
    fn new(k: i32) -> Self {
        todo!()
    }

    /** Insert an element into the circular queue. Return true if the operation is successful. */
    fn en_queue(&self, value: i32) -> bool {
        todo!()
    }

    /** Delete an element from the circular queue. Return true if the operation is successful. */
    fn de_queue(&self) -> bool {
        todo!()
    }

    /** Get the front item from the queue. */
    fn front(&self) -> i32 {
        todo!()
    }

    /** Get the last item from the queue. */
    fn rear(&self) -> i32 {
        todo!()
    }

    /** Checks whether the circular queue is empty or not. */
    fn is_empty(&self) -> bool {
        todo!()
    }

    /** Checks whether the circular queue is full or not. */
    fn is_full(&self) -> bool {
        todo!()
    }
}
