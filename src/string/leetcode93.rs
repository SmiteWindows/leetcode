// https://leetcode.com/problems/restore-ip-addresses/
pub fn restore_ip_addresses(s: String) -> Vec<String> {
    todo!()
}
// string backtracking
#[test]
#[ignore]
fn test2_93() {
    assert_eq!(
        restore_ip_addresses(String::from("25525511135")),
        vec![
            String::from("255.255.11.135"),
            String::from("255.255.111.35")
        ]
    );
}
