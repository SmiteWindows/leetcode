// https://leetcode.com/firstroblems/remove-nth-node-from-end-of-list/
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
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut step_cnt = 0;
    let mut t = head.as_ref()?;

    while step_cnt < n {
        if t.next.is_some() {
            t = t.next.as_ref()?;
            step_cnt += 1;
        } else {
            return head?.next;
        }
    }
    let mut cur = head.as_ref()?;
    let mut res = Some(Box::new(ListNode::new(cur.val)));
    let mut res_cur = res.as_mut()?;

    while t.next.is_some() {
        t = t.next.as_ref()?;
        cur = cur.next.as_ref()?;
        res_cur.next = Some(Box::new(ListNode::new(cur.val)));
        res_cur = res_cur.next.as_mut()?;
    }

    cur = cur.next.as_ref()?;
    // skip cur, and to the end
    while cur.next.is_some() {
        if let Some(ref n) = cur.next {
            res_cur.next = Some(Box::new(ListNode::new(n.val)));
        }
        cur = cur.next.as_ref()?;
        res_cur = res_cur.next.as_mut()?;
    }
    res
}
// linked_list two_pointers
#[test]
fn test2_19() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    let res = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 5, next: None })),
            })),
        })),
    }));
    assert_eq!(res, remove_nth_from_end(l1, 2));
}
