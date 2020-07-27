// https://leetcode.com/problems/palindrome-linked-list/
// Runtime: 4 ms
// Memory Usage: 3.9 MB
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let list = List::new(head);
    let vec = list.into_iter().collect::<Vec<_>>();
    for (i, &v) in vec.iter().rev().enumerate() {
        if v != vec[i] {
            return false;
        }
    }
    true
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct List {
    head: Option<Box<ListNode>>,
}

impl List {
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self { head }
    }

    fn pop(&mut self) -> Option<i32> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.val)
        } else {
            None
        }
    }

    fn into_iter(self) -> IntoIter {
        IntoIter { list: self }
    }
}

struct IntoIter {
    list: List,
}

impl Iterator for IntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
// linked_list two_pointers
#[test]
fn test2_234() {
    use leetcode_prelude::list;
    assert_eq!(is_palindrome(list![1, 2]), false);
    assert_eq!(is_palindrome(list![1, 2, 2, 1]), true);
    // let l1 = Some(Box::new(ListNode {
    //     val: 1,
    //     next: Some(Box::new(ListNode { val: 2, next: None })),
    // }));
    // let l2 = Some(Box::new(ListNode {
    //     val: 1,
    //     next: Some(Box::new(ListNode {
    //         val: 2,
    //         next: Some(Box::new(ListNode {
    //             val: 2,
    //             next: Some(Box::new(ListNode { val: 1, next: None })),
    //         })),
    //     })),
    // }));
    // assert_eq!(is_palindrome(l1), false);
    // assert_eq!(is_palindrome(l2), true);
}
