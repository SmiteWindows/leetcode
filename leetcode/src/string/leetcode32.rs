// https://leetcode-cn.com/problems/longest-valid-parentheses/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut res = 0;
    let mut stack = vec![];
    for c in s.chars() {
        if c == '(' {
            stack.push(Tok::Left)
        } else {
            match stack.pop() {
                Some(Tok::Left) => {
                    if let Some(Tok::Pair(x)) = stack.last_mut() {
                        *x += 2;
                        res = res.max(*x);
                    } else {
                        stack.push(Tok::Pair(2));
                        res = res.max(2);
                    }
                }
                Some(Tok::Pair(x)) => {
                    if let Some(Tok::Left) = stack.pop() {
                        if let Some(Tok::Pair(y)) = stack.last_mut() {
                            *y += x + 2;
                            res = res.max(*y);
                        } else {
                            stack.push(Tok::Pair(x + 2));
                            res = res.max(x + 2);
                        }
                    }
                }
                None => {}
            }
        }
    }
    if let Some(Tok::Pair(x)) = stack.pop() {
        res = res.max(x);
    }
    res
}

#[derive(Debug)]
enum Tok {
    Left,
    Pair(i32),
}
// string dynamic_programming
#[test]
fn test2_32() {
    assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
    assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
}
