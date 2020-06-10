// https://leetcode.com/problems/print-words-vertically/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn print_vertically(s: String) -> Vec<String> {
    let v = s.split_whitespace().collect::<Vec<_>>();
    let mut res = vec![];
    for (j, s) in v.iter().enumerate() {
        for (i, c) in s.char_indices() {
            if i == res.len() {
                res.push("".to_string());
            }
            while res[i].len() < j {
                res[i].push(' ');
            }
            res[i].push(c);
        }
    }
    res
}
// string
#[test]
fn test1_1324() {
    assert_eq!(
        print_vertically(String::from("HOW ARE YOU")),
        vec![
            String::from("HAY"),
            String::from("ORO"),
            String::from("WEU")
        ]
    );
    assert_eq!(
        print_vertically(String::from("TO BE OR NOT TO BE")),
        vec![
            String::from("TBONTB"),
            String::from("OEROOE"),
            String::from("   T")
        ]
    );
    assert_eq!(
        print_vertically(String::from("CONTEST IS COMING")),
        vec![
            String::from("CIC"),
            String::from("OSO"),
            String::from("N M"),
            String::from("T I"),
            String::from("E N"),
            String::from("S G"),
            String::from("T")
        ]
    );
}
