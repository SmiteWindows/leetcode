// https://leetcode-cn.com/firstroblems/remove-nth-node-from-end-of-list/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut step_cnt = 0;
    let mut t = head.as_deref()?;

    while step_cnt < n {
        if t.next.is_some() {
            t = t.next.as_deref()?;
            step_cnt += 1;
        } else {
            return head?.next;
        }
    }
    let mut cur = head.as_deref()?;
    let mut res = Some(Box::new(ListNode::new(cur.val)));
    let mut res_cur = res.as_deref_mut()?;

    while t.next.is_some() {
        t = t.next.as_deref()?;
        cur = cur.next.as_deref()?;
        res_cur.next = Some(Box::new(ListNode::new(cur.val)));
        res_cur = res_cur.next.as_deref_mut()?;
    }

    cur = cur.next.as_deref()?;
    // skip cur, and to the end
    while cur.next.is_some() {
        if let Some(ref n) = cur.next {
            res_cur.next = Some(Box::new(ListNode::new(n.val)));
        }
        cur = cur.next.as_deref()?;
        res_cur = res_cur.next.as_deref_mut()?;
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
    pub fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}
// linked_list two_pointers
#[test]
fn test2_19() {
    use leetcode_prelude::list;
    assert_eq!(
        remove_nth_from_end(list![1, 2, 3, 4, 5], 2),
        list![1, 2, 3, 5]
    );
}
