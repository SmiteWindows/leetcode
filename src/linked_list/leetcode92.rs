// https://leetcode.com/problems/reverse-linked-list-ii/
/// 反转从位置 m 到 n 的链表。请使用一趟扫描完成反转。
/// 说明: 1 ≤ m ≤ n ≤ 链表长度。
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
/// Memory Usage: 2 MB
pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
    if m == n {
        return head;
    }
    let mut cnt = 1;
    let mut header_node = Some(Box::new(ListNode { val: 0, next: head }));
    let mut ptr = header_node.as_mut()?;
    while cnt < m {
        ptr = ptr.next.as_mut()?;
        cnt += 1
    }
    let mut tail= ptr.next.take();
    let mut prev: Option<Box<ListNode>> = None;
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
        ptr = ptr.next.as_mut()?;
    }
    ptr.next = prev;
    header_node?.next
}
#[test]
fn test1_92() {
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
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    let l3 = Some(Box::new(ListNode {
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
    let l4 = Some(Box::new(ListNode {
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
    assert_eq!(l2, reverse_between(l1, 2, 4));
    assert_eq!(l4, reverse_between(l3, 2, 2));
}
