// https://leetcode.com/problems/validate-ip-address/
pub fn valid_ip_address(ip: String) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_468() {
    assert_eq!(
        valid_ip_address(String::from("172.16.254.1")),
        String::from("IPv4")
    );
    assert_eq!(
        valid_ip_address(String::from("2001:0db8:85a3:0:0:8A2E:0370:7334")),
        String::from("IPv6")
    );
    assert_eq!(
        valid_ip_address(String::from("256.256.256.256")),
        String::from("Neither")
    );
}
