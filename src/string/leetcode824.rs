// https://leetcode.com/problems/goat-latin/
pub fn to_goat_latin(s: String) -> String {
    todo!()
}
// string
#[test]
#[ignore]
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
