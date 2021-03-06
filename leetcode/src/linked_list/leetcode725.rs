// https://leetcode-cn.com/problems/split-linked-list-in-parts/
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
            write.as_mut().unwrap().next = Some(Box::new(ListNode::new(cur.unwrap().val)));
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
    use leetcode_prelude::list;
    assert_eq!(
        split_list_to_parts(list![1, 2, 3], 5),
        vec![list![1], list![2], list![3], list![], list![]]
    );
    assert_eq!(
        split_list_to_parts(list![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3),
        vec![list![1, 2, 3, 4], list![5, 6, 7], list![8, 9, 10]]
    );
}
