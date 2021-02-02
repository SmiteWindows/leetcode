// https://leetcode-cn.com/problems/add-two-numbers-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut stack1: Vec<i32> = Vec::new();
    let mut stack2: Vec<i32> = Vec::new();
    let mut res = None;
    let mut p1 = l1.as_deref();
    let mut p2 = l2.as_deref();
    while let Some(n1) = p1 {
        stack1.push(n1.val);
        p1 = n1.next.as_deref();
    }
    while let Some(n2) = p2 {
        stack2.push(n2.val);
        p2 = n2.next.as_deref();
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
// linked_list
#[test]
fn test1_445() {
    use leetcode_prelude::list;
    assert_eq!(
        add_two_numbers(list![7, 2, 4, 3], list![5, 6, 4]),
        list![7, 8, 0, 7]
    );
}
