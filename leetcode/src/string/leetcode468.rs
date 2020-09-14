// https://leetcode-cn.com/problems/validate-ip-address/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::net::{Ipv4Addr, Ipv6Addr};
pub fn valid_ip_address(ip: String) -> String {
    if is_ipv4(&ip) {
        return "IPv4".to_string();
    }
    if is_ipv6(&ip) {
        return "IPv6".to_string();
    }
    "Neither".to_string()
}

fn is_ipv4(ip: &str) -> bool {
    if let Ok(v4) = ip.parse::<Ipv4Addr>() {
        v4.to_string() == ip
    } else {
        false
    }
}

fn is_ipv6(ip: &str) -> bool {
    if ip.split(':').any(|p| p.is_empty()) {
        return false;
    }
    ip.parse::<Ipv6Addr>().is_ok()
}
// string
#[test]
fn test1_468() {
    assert_eq!(
        valid_ip_address("172.16.254.1".to_string()),
        "IPv4".to_string()
    );
    assert_eq!(
        valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_string()),
        "IPv6".to_string()
    );
    assert_eq!(
        valid_ip_address("256.256.256.256".to_string()),
        "Neither".to_string()
    );
}
