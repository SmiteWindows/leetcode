// https://leetcode.com/problems/next-greater-node-in-linked-list/
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

pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut cur = head.as_ref();
    let mut data=vec![];
    let mut res=vec![];
    let mut stack=vec![];
    let mut index=0;
    while cur.is_some() {
        res.push(0);
        data.push(cur.unwrap().val);
        // while !stack.is_empty() && cur.unwrap().val>data.get(stack.pop().unwrap()) {
        //    res.push(stack.pop().unwrap(),cur.unwrap().val);
        // }
        stack.push(index);
        index += 1;
        cur = cur.unwrap().next.as_ref();
    }
    let mut ans=Vec::with_capacity(res.len());
    for i in 0..res.len() {
        ans[i]=res[i];
    }
    ans
} 
#[test]
fn test1_1019() {
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 5, next: None })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 7,
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
        val: 1,
        next: Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode { val: 1, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        })),
    }));
    assert_eq!(next_larger_nodes(l1),vec![5,5,0]);
    assert_eq!(next_larger_nodes(l2),vec![7,0,5,5,0]);
    assert_eq!(next_larger_nodes(l3),vec![7,9,9,9,0,5,0,0]);
}
