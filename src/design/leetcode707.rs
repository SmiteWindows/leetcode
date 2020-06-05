// https://leetcode.com/problems/design-linked-list/
// Runtime: 16 ms
// Memory Usage: 2.3 MB
struct MyLinkedList {
    head: Option<Box<Node>>,
}
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl MyLinkedList {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self { head: None }
    }

    /** Get the value of the index-th node in the linked list. If the index is invalid, return -1. */
    fn get(&self, index: i32) -> i32 {
        let mut i = 0;
        let mut link = self.head.as_deref();
        while let Some(node) = link {
            if i == index {
                return node.val;
            }
            i += 1;
            link = node.next.as_deref();
        }
        -1
    }

    /** Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list. */
    fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(Node {
            val,
            next: self.head.take(),
        }));
    }

    /** Append a node of value val to the last element of the linked list. */
    fn add_at_tail(&mut self, val: i32) {
        let mut link = &mut self.head;
        while let Some(node) = link {
            link = &mut node.next;
        }
        *link = Some(Box::new(Node { val, next: None }));
    }

    /** Add a node of value val before the index-th node in the linked list. If index equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted. */
    fn add_at_index(&mut self, index: i32, val: i32) {
        if index == 0 {
            self.add_at_head(val);
        } else {
            let mut i = 0;
            let mut link = self.head.as_deref_mut();
            while let Some(node) = link {
                if index == i + 1 {
                    node.next = Some(Box::new(Node {
                        val,
                        next: node.next.take(),
                    }));
                    return;
                } else {
                    link = node.next.as_deref_mut();
                    i += 1;
                }
            }
        }
    }

    /** Delete the index-th node in the linked list, if the index is valid. */
    fn delete_at_index(&mut self, index: i32) {
        let mut i = 0;
        let mut link = &mut self.head;
        loop {
            match link {
                None => {
                    return;
                }
                Some(node) if index == i => {
                    *link = node.next.take();
                    return;
                }
                Some(node) => {
                    link = &mut node.next;
                    i += 1;
                }
            }
        }
    }
}
/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
// design linked_list
#[test]
fn test2_707() {
    let mut obj = MyLinkedList::new();
    obj.add_at_head(1);
    obj.add_at_tail(3);
    obj.add_at_index(1, 2);
    assert_eq!(obj.get(1), 2);
    obj.delete_at_index(1);
    assert_eq!(obj.get(1), 3);
}
