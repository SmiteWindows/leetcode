// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::VecDeque;
pub fn reverse_parentheses(s: String) -> String {
    let mut stack = vec![];
    let mut queue = VecDeque::new();
    for c in s.chars() {
        if c == ')' {
            while let Some(top) = stack.pop() {
                if top == '(' {
                    while let Some(front) = queue.pop_front() {
                        stack.push(front);
                    }
                    break;
                } else {
                    queue.push_back(top);
                }
            }
        } else {
            stack.push(c);
        }
    }
    stack.iter().collect()
}
// stack
#[test]
fn test1_1190() {
    assert_eq!(
        reverse_parentheses(String::from("(abcd)")),
        String::from("dcba")
    );
    assert_eq!(
        reverse_parentheses(String::from("(u(love)i)")),
        String::from("iloveu")
    );
    assert_eq!(
        reverse_parentheses(String::from("(ed(et(oc))el)")),
        String::from("leetcode")
    );
    assert_eq!(
        reverse_parentheses(String::from("a(bcdefghijkl(mno)p)q")),
        String::from("apmnolkjihgfedcbq")
    );
}
