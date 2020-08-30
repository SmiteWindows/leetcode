// https://leetcode-cn.com/problems/next-greater-node-in-linked-list/
// Runtime: 32 ms
// Memory Usage: 2.6 MB
pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut head = head;
    let mut nodes = vec![];
    while let Some(node) = head {
        nodes.push(node.val);
        head = node.next;
    }
    let n = nodes.len();
    let mut stack = vec![];
    let mut res = vec![0; n];
    for i in 0..n {
        while let Some(j) = stack.pop() {
            if nodes[j] < nodes[i] {
                res[j] = nodes[i];
            } else {
                stack.push(j);
                break;
            }
        }
        stack.push(i);
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
// linked_list stack
#[test]
fn test1_1019() {
    use leetcode_prelude::list;
    assert_eq!(next_larger_nodes(list![2, 1, 5]), vec![5, 5, 0]);
    assert_eq!(next_larger_nodes(list![2, 7, 4, 3, 5]), vec![7, 0, 5, 5, 0]);
    assert_eq!(
        next_larger_nodes(list![1, 7, 5, 1, 9, 2, 5, 1]),
        vec![7, 9, 9, 9, 0, 5, 0, 0]
    );
}
