// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut root = None; //head(node) ,(head)node.next ,last_node
    let mut p = &mut root;
    let mut last_node: Option<Box<ListNode>> = None;
    let mut last_duplicate = false;
    while let Some(mut node) = head {
        match last_node {
            Some(old_node) if old_node.val == node.val => {
                head = node.next;
                node.next = None;
                last_node = Some(node);

                last_duplicate = true;
            }
            Some(old_node) => {
                head = node.next;
                node.next = None;
                last_node = Some(node);

                if last_duplicate {
                    last_duplicate = false;
                    continue;
                }

                *p = Some(old_node);

                if let Some(tmp_node) = p {
                    p = &mut tmp_node.next;
                }
                last_duplicate = false;
            }
            None => {
                head = node.next;
                node.next = None;
                last_node = Some(node);
            }
        }
    }
    if !last_duplicate {
        if let Some(node) = last_node {
            *p = Some(node);
        }
    }
    root
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
fn test1_82() {
    use leetcode_prelude::list;
    assert_eq!(
        delete_duplicates(list![1, 2, 3, 3, 4, 4, 5]),
        list![1, 2, 5]
    );
    assert_eq!(delete_duplicates(list![1, 1, 1, 2, 3]), list![2, 3]);
}
