// https://leetcode-cn.com/problems/add-two-numbers/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (mut lhs, mut rhs) = (l1, l2);
    let mut head = ListNode::new(0);
    let mut curr = &mut head;
    let mut carry = 0;
    loop {
        match (lhs, rhs) {
            (Some(mut v1), Some(mut v2)) => {
                let mut val = v1.val + v2.val + carry;
                if val >= 10 {
                    val -= 10;
                    carry = 1;
                } else {
                    carry = 0;
                }
                let node = ListNode::new(val);
                curr.next = Some(Box::new(node));
                lhs = v1.next.take();
                rhs = v2.next.take();
            }
            (Some(mut v1), None) => {
                let mut val = v1.val + carry;
                if val >= 10 {
                    val -= 10;
                    carry = 1;
                } else {
                    carry = 0;
                }
                let node = ListNode::new(val);
                curr.next = Some(Box::new(node));
                lhs = v1.next.take();
                rhs = None;
            }
            (None, Some(mut v2)) => {
                let mut val = v2.val + carry;
                if val >= 10 {
                    val -= 10;
                    carry = 1;
                } else {
                    carry = 0;
                }
                let node = ListNode::new(val);
                curr.next = Some(Box::new(node));
                rhs = v2.next.take();
                lhs = None;
            }
            (None, None) => {
                if carry == 0 {
                    break;
                }
                let node = ListNode::new(1);
                curr.next = Some(Box::new(node));
                break;
            }
        }
        curr = curr.next.as_deref_mut()?;
    }
    head.next
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}
// linked_list math
#[test]
fn test2_2() {
    use leetcode_prelude::list;
    assert_eq!(
        add_two_numbers(list![2, 4, 3], list![5, 6, 4]),
        list![7, 0, 8]
    );
}
