// https://leetcode-cn.com/problems/backspace-string-compare/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn backspace_compare(s: String, t: String) -> bool {
    let mut s_i = s.chars().rev();
    let mut t_i = t.chars().rev();
    loop {
        let s_char = get_next(&mut s_i);
        let t_char = get_next(&mut t_i);
        if s_char == None && t_char == None {
            break true;
        }
        if s_char != t_char {
            break false;
        }
    }
}

fn get_next(iter: &mut impl Iterator<Item = char>) -> Option<char> {
    let mut skip = 0;
    loop {
        let cur = iter.next();
        match cur {
            Some('#') => skip += 1,
            None => break None,
            Some(x) => {
                if skip == 0 {
                    break Some(x);
                }
                skip -= 1;
            }
        }
    }
}
// two_pointers stack
#[test]
fn test1_844() {
    assert_eq!(
        backspace_compare(String::from("ab#c"), String::from("ad#c")),
        true
    );
    assert_eq!(
        backspace_compare(String::from("ab##"), String::from("c#d#")),
        true
    );
    assert_eq!(
        backspace_compare(String::from("a##c"), String::from("#a#c")),
        true
    );
    assert_eq!(
        backspace_compare(String::from("a#c"), String::from("b")),
        false
    );
}
