// https://leetcode.com/problems/print-words-vertically/
pub fn print_vertically(s: String) -> Vec<String> {
    todo!()
}
// string
#[test]
#[ignore]
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
