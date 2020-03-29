// https://leetcode.com/problems/add-two-numbers/
/// 给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。
/// 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
/// 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
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
/// Runtime: 4 ms
/// Memory Usage: 2.2 MB
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
        curr = curr.next.as_mut()?;
    }

    head.next
}
#[test]
fn test2_2(){
    let mut first = Some(Box::new(ListNode::new(3)));
    first.as_mut().unwrap().next = None;
    let mut second = Some(Box::new(ListNode::new(4)));
    second.as_mut().unwrap().next = first;
    let mut left = Some(Box::new(ListNode::new(2)));
    left.as_mut().unwrap().next = second;

    let mut first = Some(Box::new(ListNode::new(4)));
    first.as_mut().unwrap().next = None;
    let mut second = Some(Box::new(ListNode::new(6)));
    second.as_mut().unwrap().next = first;
    let mut right = Some(Box::new(ListNode::new(5)));
    right.as_mut().unwrap().next = second;

    let mut first = Some(Box::new(ListNode::new(8)));
    first.as_mut().unwrap().next = None;
    let mut second = Some(Box::new(ListNode::new(0)));
    second.as_mut().unwrap().next = first;
    let mut res = Some(Box::new(ListNode::new(7)));
    res.as_mut().unwrap().next = second;

    assert_eq!(res, add_two_numbers(left, right));
}