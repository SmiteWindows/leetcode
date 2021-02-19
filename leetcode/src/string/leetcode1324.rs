// https://leetcode-cn.com/problems/print-words-vertically/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn print_vertically(s: String) -> Vec<String> {
    let v = s.split_whitespace().collect::<Vec<_>>();
    let mut res = Vec::new();
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
    use leetcode_prelude::vec_string;
    assert_eq!(
        print_vertically("HOW ARE YOU".to_string()),
        vec_string!["HAY", "ORO", "WEU"]
    );
    assert_eq!(
        print_vertically("TO BE OR NOT TO BE".to_string()),
        vec_string!["TBONTB", "OEROOE", "   T"]
    );
    assert_eq!(
        print_vertically("CONTEST IS COMING".to_string()),
        vec_string!["CIC", "OSO", "N M", "T I", "E N", "S G", "T"]
    );
}
