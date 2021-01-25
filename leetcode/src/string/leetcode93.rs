// https://leetcode-cn.com/problems/restore-ip-addresses/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let n = s.len();
    let mut v = vec![0_u8; 4];
    let mut res = vec![];
    for i in 1..4 {
        if i + 3 > n {
            break;
        }
        if let Ok(a) = s[0..i].parse::<u8>() {
            v[0] = a;
        } else {
            break;
        }
        for j in 1..4 {
            if i + j + 2 > n {
                break;
            }
            if let Ok(b) = s[i..i + j].parse::<u8>() {
                v[1] = b;
            } else {
                break;
            }
            for k in 1..4 {
                if i + j + k + 1 > n {
                    break;
                }
                if let Ok(c) = s[i + j..i + j + k].parse::<u8>() {
                    v[2] = c;
                } else {
                    break;
                }
                for l in 1..4 {
                    if i + j + k + l != n {
                        continue;
                    }
                    if let Ok(d) = s[i + j + k..n].parse::<u8>() {
                        v[3] = d;
                        let ip = format!("{}.{}.{}.{}", v[0], v[1], v[2], v[3]);
                        if ip.len() == n + 3 {
                            res.push(ip);
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
    res
}
// string backtracking
#[test]
fn test2_93() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        restore_ip_addresses("25525511135".to_string()),
        vec_string!["255.255.11.135", "255.255.111.35"]
    );
}
