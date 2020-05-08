// https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/
pub fn remove_duplicates(s: String) -> String {
    let mut stack = vec![];
    for c in s.chars() {
        if let Some(&top) = stack.last() {
            if top == c {
                stack.pop();
            } else {
                stack.push(c);
            }
        } else {
            stack.push(c)
        }
    }
    stack.iter().collect()
}
// stack
#[test]
fn test1_1047() {
    assert_eq!(
        remove_duplicates(String::from("abbaca")),
        String::from("ca")
    );
}
