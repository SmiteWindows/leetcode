// https://leetcode.com/problems/add-two-numbers-ii/
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut stack1: Vec<i32> = vec![];
    let mut stack2: Vec<i32> = vec![];
    let mut res: Option<Box<ListNode>> = None;
    let mut p1 = &l1;
    let mut p2 = &l2;
    while let Some(n1) = p1 {
        stack1.push(n1.val);
        p1 = &n1.next;
    }
    while let Some(n2) = p2 {
        stack2.push(n2.val);
        p2 = &n2.next;
    }
    let mut carry = 0;
    while !stack1.is_empty() || !stack2.is_empty() || carry != 0 {
        let mut sum = 0;
        if let Some(x1) = stack1.pop() {
            sum += x1;
        }
        if let Some(x2) = stack2.pop() {
            sum += x2;
        }
        sum += carry;
        res = Some(Box::new(ListNode {
            val: sum % 10,
            next: res,
        }));
        carry = sum / 10;
    }
    res
}
#[test]
fn test1_445() {
    let l1 = Some(Box::new(ListNode {
        val: 7,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let res = Some(Box::new(ListNode {
        val: 7,
        next: Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 7, next: None })),
            })),
        })),
    }));
    assert_eq!(res, add_two_numbers(l1, l2));
}
