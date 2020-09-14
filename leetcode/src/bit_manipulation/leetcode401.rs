// https://leetcode-cn.com/problems/binary-watch/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn read_binary_watch(num: i32) -> Vec<String> {
    let mut res = vec![];
    for i in 0..12 {
        for j in 0..60 {
            if i32::count_ones(i) + i32::count_ones(j) == num as u32 {
                res.push(format!("{}:{:02}", i, j));
            }
        }
    }
    res
}
// backtracking bit_manipulation
#[test]
fn test2_401() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        read_binary_watch(1),
        vec_string!["0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"]
    );
}
