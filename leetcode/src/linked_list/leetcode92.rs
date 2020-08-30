// https://leetcode-cn.com/problems/reverse-linked-list-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
    if m == n {
        return head;
    }
    let mut cnt = 1;
    let mut header_node = Some(Box::new(ListNode { val: 0, next: head }));
    let mut ptr = header_node.as_deref_mut()?;
    while cnt < m {
        ptr = ptr.next.as_deref_mut()?;
        cnt += 1
    }
    let mut tail = ptr.next.take();
    let mut prev = None;
    if let Some(mut node) = tail {
        prev = node.next.take();
        tail = Some(node);
    }
    while let Some(mut node) = prev {
        prev = node.next.take();
        node.next = tail;
        tail = Some(node);
        cnt += 1;
        if cnt == n {
            break;
        }
    }
    ptr.next = tail;
    while ptr.next.is_some() {
        ptr = ptr.next.as_deref_mut()?;
    }
    ptr.next = prev;
    header_node?.next
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
fn test1_92() {
    use leetcode_prelude::list;
    assert_eq!(
        reverse_between(list![1, 2, 3, 4, 5], 2, 4),
        list![1, 4, 3, 2, 5]
    );
}
