// https://leetcode.com/problems/crawler-log-folder/
pub fn min_operations(logs: Vec<String>) -> i32 {
    let mut res = 0;
    for log in logs {
        if log == "../" {
            if res > 0 {
                res -= 1;
            }
        } else if log != "./" {
            res += 1;
        }
    }
    res
}
// Runtime: 0 ms
// Memory Usage: 2.1 MB
// stack
#[test]
fn test1_1598() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        min_operations(vec_string!["d1/", "d2/", "../", "d21/", "./"]),
        2
    );
    assert_eq!(
        min_operations(vec_string!["d1/", "d2/", "./", "d3/", "../", "d31/"]),
        3
    );
    assert_eq!(min_operations(vec_string!["d1/", "../", "../", "../"]), 0);
}
