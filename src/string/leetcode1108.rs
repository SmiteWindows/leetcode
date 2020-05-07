// https://leetcode.com/problems/defanging-an-ip-address/
pub fn defang_i_paddr(address: String) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1108() {
    assert_eq!(
        defang_i_paddr(String::from("1.1.1.1")),
        String::from("1[.]1[.]1[.]1")
    );
    assert_eq!(
        defang_i_paddr(String::from("255.100.50.0")),
        String::from("255[.]100[.]50[.]0")
    );
}
