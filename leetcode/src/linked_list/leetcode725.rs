// https://leetcode.com/problems/split-linked-list-in-parts/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn split_list_to_parts(root: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    let mut cur = root.as_deref();
    let k = k as usize;
    let mut n = 0;
    while let Some(x) = cur {
        cur = x.next.as_deref();
        n += 1;
    }
    let width = n / k;
    let rem = n % k;
    let mut res = Vec::with_capacity(k);
    cur = root.as_deref();
    let mut i = 0;
    while i < k {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut write = head.as_deref_mut();
        let mut j = 0;
        while j < width + if i < rem { 1 } else { 0 } {
            write.as_mut().unwrap().next =
                Some(Box::new(ListNode::new(cur.unwrap().val)));
            write = write.unwrap().next.as_deref_mut();
            if let Some(x) = cur {
                cur = x.next.as_deref();
            }
            j += 1;
        }
        res.push(head.unwrap().next);
        i += 1;
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
fn test1_725() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let l2 = vec![
        Some(Box::new(ListNode { val: 1, next: None })),
        Some(Box::new(ListNode { val: 2, next: None })),
        Some(Box::new(ListNode { val: 3, next: None })),
        None,
        None,
    ];
    let l3 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode {
                            val: 6,
                            next: Some(Box::new(ListNode {
                                val: 7,
                                next: Some(Box::new(ListNode {
                                    val: 8,
                                    next: Some(Box::new(ListNode {
                                        val: 9,
                                        next: Some(Box::new(ListNode {
                                            val: 10,
                                            next: None,
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        })),
    }));
    let l4 = vec![
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        })),
        Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 7, next: None })),
            })),
        })),
        Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 10,
                    next: None,
                })),
            })),
        })),
    ];
    assert_eq!(l2, split_list_to_parts(l1, 5));
    assert_eq!(l4, split_list_to_parts(l3, 3));
}
