// https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut cur = head.as_deref();
    let mut res = 0;
    while let Some(node) = cur {
        res = res * 2 + node.val;
        cur = node.next.as_deref();
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
// linked_list bit_manipulation
#[test]
fn test1_1290() {
    use leetcode_prelude::list;
    assert_eq!(get_decimal_value(list![1, 0, 1]), 5);
    assert_eq!(get_decimal_value(list![0]), 0);
    assert_eq!(get_decimal_value(list![1]), 1);
    assert_eq!(
        get_decimal_value(list![1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]),
        18880
    );
    assert_eq!(get_decimal_value(list![0, 0]), 0);
}
