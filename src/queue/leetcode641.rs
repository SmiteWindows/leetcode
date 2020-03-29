// https://leetcode.com/problems/design-circular-deque/
struct MyCircularDeque {}

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
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    /** Initialize your data structure here. Set the size of the deque to be k. */
    fn new(k: i32) -> Self {
        todo!()
    }

    /** Adds an item at the front of Deque. Return true if the operation is successful. */
    fn insert_front(&self, value: i32) -> bool {
        todo!()
    }

    /** Adds an item at the rear of Deque. Return true if the operation is successful. */
    fn insert_last(&self, value: i32) -> bool {
        todo!()
    }

    /** Deletes an item from the front of Deque. Return true if the operation is successful. */
    fn delete_front(&self) -> bool {
        todo!()
    }

    /** Deletes an item from the rear of Deque. Return true if the operation is successful. */
    fn delete_last(&self) -> bool {
        todo!()
    }

    /** Get the front item from the deque. */
    fn get_front(&self) -> i32 {
        todo!()
    }

    /** Get the last item from the deque. */
    fn get_rear(&self) -> i32 {
        todo!()
    }

    /** Checks whether the circular deque is empty or not. */
    fn is_empty(&self) -> bool {
        todo!()
    }

    /** Checks whether the circular deque is full or not. */
    fn is_full(&self) -> bool {
        todo!()
    }
}
