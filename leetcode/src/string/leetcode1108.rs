// https://leetcode-cn.com/problems/defanging-an-ip-address/
pub fn defang_i_paddr(address: String) -> String {
    address.replace(".", "[.]")
}
// Runtime: 0 ms
// Memory Usage: 2 MB
// ✔
// string
#[test]
fn test1_1108() {
    assert_eq!(
        defang_i_paddr("1.1.1.1".to_string()),
        "1[.]1[.]1[.]1".to_string()
    );
    assert_eq!(
        defang_i_paddr("255.100.50.0".to_string()),
        "255[.]100[.]50[.]0".to_string()
    );
}
