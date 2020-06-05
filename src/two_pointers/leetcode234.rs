// https://leetcode.com/problems/palindrome-linked-list/

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    todo!()
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
#[ignore]
fn test1_234() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode { val: 2, next: None })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        })),
    }));
    assert_eq!(is_palindrome(l1), false);
    assert_eq!(is_palindrome(l2), true);
}
