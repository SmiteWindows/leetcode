// https://leetcode.com/problems/reverse-nodes-in-k-group/
/// 给你一个链表，每 k 个节点一组进行翻转，请你返回翻转后的链表。
/// k 是一个正整数，它的值小于或等于链表的长度。
/// 如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。
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
/// Runtime: 0 ms
/// Memory Usage: 2.3 MB
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    fn reverse(
        mut head: Option<Box<ListNode>>,
        mut tail: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        while let Some(mut current) = head {
            let next = current.next.take();
            current.next = tail.take();
            tail = Some(current);
            head = next;
        }
        tail
    }
    let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
    let mut head = dummy_head.as_mut();
    'a: loop {
        let mut start = head.as_mut()?.next.take();
        if start.is_none() {
            break;
        }
        let mut end = start.as_mut();
        for _ in 0..k - 1 {
            end = end?.next.as_mut();
            if end.is_none() {
                head.as_mut()?.next = start;
                break 'a;
            }
        }
        let tail = end.as_mut()?.next.take();
        let end = reverse(start, tail);
        head.as_mut()?.next = end;
        for _ in 0..k {
            head = head?.next.as_mut();
        }
    }
    dummy_head?.next
}
#[test]
fn test1_25() {
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
    let l12 = l1.clone();
    let l2 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    let l3 = Some(Box::new(ListNode {
        val: 3,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    assert_eq!(l2, reverse_k_group(l1, 2));
    assert_eq!(l3, reverse_k_group(l12, 3));
}
