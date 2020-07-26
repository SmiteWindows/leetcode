// https://leetcode.com/problems/add-two-numbers/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (mut lhs, mut rhs) = (l1, l2);
    let mut res = Some(Box::new(ListNode::new(0)));
    let (mut tmp, mut sum) = (&mut res, 0);
    let (mut lhs_tag, mut rhs_tag) = (lhs.is_some(), rhs.is_some());
    while lhs_tag || rhs_tag || sum > 0 {
        if lhs_tag {
            sum += lhs.as_deref()?.val;
            lhs = lhs?.next;
            lhs_tag = lhs.is_some();
        }
        if rhs_tag {
            sum += rhs.as_deref()?.val;
            rhs = rhs?.next;
            rhs_tag = rhs.is_some();
        }
        tmp.as_deref_mut()?.next = Some(Box::new(ListNode::new(sum % 10)));
        tmp = &mut tmp.as_deref_mut()?.next;
        sum /= 10;
    }
    res?.next
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
// linked_list math
#[test]
fn test1_2() {
    use leetcode_prelude::list;
    assert_eq!(
        add_two_numbers(list![2, 4, 3], list![5, 6, 4]),
        list![7, 0, 8]
    );
    // let l1 = Some(Box::new(ListNode {
    //     val: 2,
    //     next: Some(Box::new(ListNode {
    //         val: 4,
    //         next: Some(Box::new(ListNode { val: 3, next: None })),
    //     })),
    // }));
    // let l2 = Some(Box::new(ListNode {
    //     val: 5,
    //     next: Some(Box::new(ListNode {
    //         val: 6,
    //         next: Some(Box::new(ListNode { val: 4, next: None })),
    //     })),
    // }));
    // let res = Some(Box::new(ListNode {
    //     val: 7,
    //     next: Some(Box::new(ListNode {
    //         val: 0,
    //         next: Some(Box::new(ListNode { val: 8, next: None })),
    //     })),
    // }));
    // assert_eq!(res, add_two_numbers(l1, l2));
}
