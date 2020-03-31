// https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/
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
/// Memory Usage: 1.9 MB
pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut cur = head.as_ref();
    let mut res = 0;
    while cur.is_some() {
        res = res << 1 | cur.unwrap().val;
        cur = cur.unwrap().next.as_ref();
    }
    res
}
#[test]
fn test2_1290() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        })),
    }));
    let l2 = Some(Box::new(ListNode { val: 0, next: None }));
    let l3 = Some(Box::new(ListNode { val: 1, next: None }));
    let l4 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 1,
                                next: Some(Box::new(ListNode {
                                    val: 1,
                                    next: Some(Box::new(ListNode {
                                        val: 1,
                                        next: Some(Box::new(ListNode {
                                            val: 0,
                                            next: Some(Box::new(ListNode {
                                                val: 0,
                                                next: Some(Box::new(ListNode {
                                                    val: 0,
                                                    next: Some(Box::new(ListNode {
                                                        val: 0,
                                                        next: Some(Box::new(ListNode {
                                                            val: 0,
                                                            next: Some(Box::new(ListNode {
                                                                val: 0,
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
                        })),
                    })),
                })),
            })),
        })),
    }));
    let l5 = Some(Box::new(ListNode {
        val: 0,
        next: Some(Box::new(ListNode { val: 0, next: None })),
    }));
    assert_eq!(get_decimal_value(l1), 5);
    assert_eq!(get_decimal_value(l2), 0);
    assert_eq!(get_decimal_value(l3), 1);
    assert_eq!(get_decimal_value(l4), 18880);
    assert_eq!(get_decimal_value(l5), 0);
}