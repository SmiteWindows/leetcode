// https://leetcode-cn.com/problems/reverse-substrings-between-each-pair-of-parentheses/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::VecDeque;
pub fn reverse_parentheses(s: String) -> String {
    let mut stack = Vec::new();
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
        reverse_parentheses("(abcd)".to_string()),
        "dcba".to_string()
    );
    assert_eq!(
        reverse_parentheses("(u(love)i)".to_string()),
        "iloveu".to_string()
    );
    assert_eq!(
        reverse_parentheses("(ed(et(oc))el)".to_string()),
        "leetcode".to_string()
    );
    assert_eq!(
        reverse_parentheses("a(bcdefghijkl(mno)p)q".to_string()),
        "apmnolkjihgfedcbq".to_string()
    );
}
