// https://leetcode-cn.com/problems/odd-even-linked-list/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut odd = None;
    let (mut odd_cur, mut even_cur) = (&mut odd, &mut head);
    while odd_cur.is_some() || even_cur.is_some() {
        *odd_cur = even_cur.take();
        odd_cur = &mut odd_cur.as_deref_mut()?.next;
        if odd_cur.is_none() {
            break;
        }
        *even_cur = odd_cur.take();
        even_cur = &mut even_cur.as_deref_mut()?.next;
    }
    *odd_cur = head;
    odd
    // let mut odd_list = Some(Box::new(ListNode::new(0)));
    // let mut even_list = Some(Box::new(ListNode::new(0)));
    // let mut odd_tail = &mut odd_list;
    // let mut even_tail = &mut even_list;
    // let mut curr = head;
    // let mut is_odd = true;
    // while let Some(mut curr_inner) = curr {
    //     curr = curr_inner.next.take();
    //     if is_odd {
    //         odd_tail.as_mut()?.next = Some(curr_inner);
    //         odd_tail = &mut odd_tail.as_mut()?.next;
    //     } else {
    //         even_tail.as_mut()?.next = Some(curr_inner);
    //         even_tail = &mut even_tail.as_mut()?.next;
    //     }
    //     is_odd = !is_odd;
    // }
    // odd_tail.as_mut()?.next = even_list?.next.take();
    // odd_list?.next
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
fn test1_328() {
    use leetcode_prelude::list;
    assert_eq!(odd_even_list(list![1, 2, 3, 4, 5]), list![1, 3, 5, 2, 4]);
    assert_eq!(
        odd_even_list(list![2, 1, 3, 5, 6, 4, 7]),
        list![2, 3, 6, 7, 1, 5, 4]
    );
}
