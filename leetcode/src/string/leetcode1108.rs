// https://leetcode-cn.com/problems/defanging-an-ip-address/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn defang_i_paddr(address: String) -> String {
    address.replace(".", "[.]")
}
// string
#[test]
fn test1_1108() {
    assert_eq!(
        defang_i_paddr("1.1.1.1")),
        "1[.]1[.]1[.]1")
    );
    assert_eq!(
        defang_i_paddr("255.100.50.0")),
        "255[.]100[.]50[.]0")
    );
}
