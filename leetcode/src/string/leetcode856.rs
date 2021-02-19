// https://leetcode-cn.com/problems/score-of-parentheses/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn score_of_parentheses(s: String) -> i32 {
    let mut stack = Vec::new();
    for c in s.chars() {
        if c == '(' {
            stack.push(0);
        } else {
            let mut sum = 0;
            while let Some(last) = stack.pop() {
                if last != 0 {
                    sum += last;
                } else {
                    break;
                }
            }
            if sum == 0 {
                stack.push(1);
            } else {
                stack.push(2 * sum);
            }
        }
    }
    stack.iter().sum()
}
// stack string
#[test]
fn test2_856() {
    assert_eq!(score_of_parentheses("()".to_string()), 1);
    assert_eq!(score_of_parentheses("(())".to_string()), 2);
    assert_eq!(score_of_parentheses("()()".to_string()), 2);
    assert_eq!(score_of_parentheses("(()(()))".to_string()), 6);
}
