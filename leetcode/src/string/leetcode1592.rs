// https://leetcode.com/problems/rearrange-spaces-between-words/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn reorder_spaces(text: String) -> String {
    let space = text.chars().filter(|&c| c == ' ').count();
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    if words.len() == 1 {
        let mut res = "".to_string();
        res.push_str(&words[0]);
        for _ in 0..space {
            res.push(' ');
        }
        res
    } else {
        let gap = words.len() - 1;
        let width = space / gap;
        let left = space - width * gap;
        let mut res = "".to_string();
        for (i, word) in words.iter().enumerate() {
            res.push_str(word);
            for _ in 0..if i < gap { width } else { left } {
                res.push(' ');
            }
        }
        res
    }
}
// string
#[test]
fn test1_1592() {
    assert_eq!(
        reorder_spaces("  this   is  a sentence ".to_string()),
        "this   is   a   sentence".to_string()
    );
    assert_eq!(
        reorder_spaces(" practice   makes   perfect".to_string()),
        "practice   makes   perfect ".to_string()
    );
    assert_eq!(
        reorder_spaces("hello   world".to_string()),
        "hello   world".to_string()
    );
    assert_eq!(
        reorder_spaces("  walks  udp package   into  bar a".to_string()),
        "walks  udp  package  into  bar  a ".to_string()
    );
    assert_eq!(reorder_spaces("a".to_string()), "a".to_string());
}
