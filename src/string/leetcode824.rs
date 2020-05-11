// https://leetcode.com/problems/goat-latin/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn to_goat_latin(s: String) -> String {
    let words: Vec<&str> = s.split_whitespace().map(|s| s.chars().as_str()).collect();
    let mut res: String = "".to_string();
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
        to_goat_latin(String::from("I speak Goat Latin")),
        String::from("Imaa peaksmaaa oatGmaaaa atinLmaaaaa")
    );
    assert_eq!(
        to_goat_latin(String::from("The quick brown fox jumped over the lazy dog")),
        String::from("heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa")
    );
}
