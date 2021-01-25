// https://leetcode-cn.com/problems/goat-latin/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn to_goat_latin(s: String) -> String {
    let words = s
        .split_whitespace()
        .map(|s| s.chars().as_str())
        .collect::<Vec<_>>();
    let mut res = "".to_string();
    let mut n = 1;
    for word in words {
        if n > 1 {
            res += " ";
        }
        match &word[0..1] {
            "a" | "e" | "i" | "o" | "u" | "A" | "E" | "I" | "O" | "U" => {
                res += word;
            }
            _ => {
                res += &word[1..];
                res += &word[0..1];
            }
        }
        res += "ma";
        for _ in 0..n {
            res += "a";
        }
        n += 1;
    }
    res
}
// string
#[test]
fn test1_824() {
    assert_eq!(
        to_goat_latin("I speak Goat Latin".to_string()),
        "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string()
    );
    assert_eq!(
        to_goat_latin("The quick brown fox jumped over the lazy dog".to_string()),
        "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa".to_string()
    );
}
