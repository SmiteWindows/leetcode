// https://leetcode-cn.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/
// Runtime: 4 ms
// Memory Usage: 2 MB
use std::collections::HashMap;
pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut stack = vec![];
    let mut hm = HashMap::new();
    let mut sum = 0;
    hm.insert(0, 0);
    while let Some(node) = head {
        let val = node.val;
        head = node.next;
        if val == 0 {
            continue;
        }
        sum += val;
        if let Some(&size) = hm.get(&sum) {
            sum -= val;
            while stack.len() > size {
                let top = stack.pop().unwrap();
                hm.remove(&sum);
                sum -= top;
            }
        } else {
            stack.push(val);
            hm.insert(sum, stack.len());
        }
    }
    let mut prev = None;
    while let Some(top) = stack.pop() {
        prev = Some(Box::new(ListNode {
            val: top,
            next: prev,
        }));
    }
    prev
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
fn test1_1171() {
    use leetcode_prelude::list;
    assert_eq!(remove_zero_sum_sublists(list![1, 2, -3, 3, 1]), list![3, 1]);
    assert_eq!(
        remove_zero_sum_sublists(list![1, 2, 3, -3, 4]),
        list![1, 2, 4]
    );
    assert_eq!(remove_zero_sum_sublists(list![1, 2, 3, -3, -2]), list![1]);
}
