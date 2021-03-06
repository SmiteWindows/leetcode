// https://leetcode-cn.com/problems/sort-list/
// Runtime: 8 ms
// Memory Usage: 4.2 MB
pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.as_ref()?;
    let mut len = 1;
    loop {
        let mut new_head = None;
        let mut new_tail = None;
        let mut loop_count = 0;
        loop {
            loop_count += 1;
            let mut take_head = None;
            let mut take_tail = None;
            for _ in 0..len {
                match head {
                    Some(mut t) => {
                        head = t.next.take();
                        if take_tail.is_none() {
                            take_head = Some(t);
                            take_tail = take_head.as_deref_mut();
                        } else {
                            take_tail = take_tail
                                .map(|p| {
                                    p.next = Some(t);
                                    p.next.as_deref_mut()
                                })
                                .flatten();
                        }
                    }
                    None => break,
                }
            }
            let mut count = len;
            loop {
                let mut pick = None;
                if take_head.is_some() && head.is_some() && count > 0 {
                    if take_head.as_deref()?.val <= head.as_deref()?.val {
                        pick = take_head;
                        take_head = pick.as_deref_mut()?.next.take();
                    } else {
                        count -= 1;
                        pick = head;
                        head = pick.as_deref_mut()?.next.take();
                    }
                } else if take_head.is_some() {
                    pick = take_head;
                    take_head = pick.as_deref_mut()?.next.take();
                } else if head.is_some() && count > 0 {
                    count -= 1;
                    pick = head;
                    head = pick.as_deref_mut()?.next.take();
                }
                if pick.is_none() {
                    break;
                }
                if new_tail.is_none() {
                    new_head = pick;
                    new_tail = new_head.as_deref_mut();
                } else {
                    new_tail = new_tail
                        .map(|p| {
                            p.next = pick;
                            p.next.as_deref_mut()
                        })
                        .flatten();
                }
            }
            if count > 0 {
                break;
            }
        }
        head = new_head;
        len *= 2;
        if loop_count <= 1 {
            break;
        }
    }
    head
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
// linked_list sort
#[test]
fn test2_148() {
    let l1 = Some(Box::new(ListNode {
        val: 4,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
    }));
    assert_eq!(l2, sort_list(l1));
    let l3 = Some(Box::new(ListNode {
        val: -1,
        next: Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 0, next: None })),
                })),
            })),
        })),
    }));
    let l4 = Some(Box::new(ListNode {
        val: -1,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    assert_eq!(l4, sort_list(l3));
}
