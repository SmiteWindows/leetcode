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
fn test1_401() {
    assert_eq!(
        read_binary_watch(1),
        vec![
            String::from("0:01"),
            String::from("0:02"),
            String::from("0:04"),
            String::from("0:08"),
            String::from("0:16"),
            String::from("0:32"),
            String::from("1:00"),
            String::from("2:00"),
            String::from("4:00"),
            String::from("8:00")
        ]
    );
}
