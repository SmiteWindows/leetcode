// https://leetcode.com/problems/odd-even-linked-list/
/// 给定一个单链表，把所有的奇数节点和偶数节点分别排在一起。
/// 请注意，这里的奇数节点和偶数节点指的是节点编号的奇偶性，而不是节点的值的奇偶性。
/// 请尝试使用原地算法完成。你的算法的空间复杂度应为 O(1)，时间复杂度应为 O(nodes)，nodes 为节点总数。
/// 应当保持奇数节点和偶数节点的相对顺序。
/// 链表的第一个节点视为奇数节点，第二个节点视为偶数节点，以此类推。
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
pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut odd_list = Some(Box::new(ListNode::new(0)));
    let mut even_list = Some(Box::new(ListNode::new(0)));
    let mut odd_tail = &mut odd_list;
    let mut even_tail = &mut even_list;
    let mut curr = head;
    let mut is_odd=true;
    while let Some(mut curr_inner) = curr {
        curr = curr_inner.next.take();
        if is_odd {
            odd_tail.as_mut()?.next =Some(curr_inner);
            odd_tail=&mut odd_tail.as_mut()?.next;
        }
        else {
            even_tail.as_mut()?.next =Some(curr_inner);
            even_tail=&mut even_tail.as_mut()?.next;
        }
        is_odd=!is_odd;
    }
    odd_tail.as_mut()?.next = even_list?.next.take();
    odd_list?.next
}
#[test]
fn test1_328() {
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
            val: 3,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        })),
    }));
    let l3 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 7, next: None })),
                        })),
                    })),
                })),
            })),
        })),
    }));
    let l4 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 7,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        })),
    }));
    assert_eq!(odd_even_list(l1), l2);
    assert_eq!(odd_even_list(l3), l4);
}