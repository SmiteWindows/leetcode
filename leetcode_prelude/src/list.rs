/// Definition for singly-linked list.
///
/// # Note
///
/// I add Ord PartialOrd for sort Vec<TreeNode> when testing
/// Please don't rely on it
use std::fmt;

#[derive(PartialEq, Eq, Clone, Ord, PartialOrd)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

impl fmt::Debug for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = vec![self.val];
        let mut p = self;
        while let Some(next) = p.next.as_deref() {
            v.push(next.val);
            p = next;
        }
        write!(f, "{:?}", v)
    }
}

/// Create a linked list with ListNode
///
/// # Example
///
/// ```rust
/// use leetcode_prelude::list;
/// use leetcode_prelude::ListNode;
///
/// let list = list![1, 2, 3];
/// ```
#[macro_export]
macro_rules! list {
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            let mut head = Box::new(ListNode::new(0));
            let mut ref_head = &mut head;
            $(
            ref_head.next = Some(Box::new(ListNode::new($e)));
            ref_head = ref_head.next.as_mut().unwrap();
            )*
            head.next
        }
    };
}
