// https://leetcode-cn.com/problems/restore-ip-addresses/
pub fn restore_ip_addresses(s: String) -> Vec<String> {
    todo!()
}
// string backtracking
#[test]
#[ignore]
fn test1_93() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        restore_ip_addresses("25525511135".to_string()),
        vec_string!["255.255.11.135", "255.255.111.35"]
    );
}
