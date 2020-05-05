// https://leetcode.com/problems/binary-watch/
pub fn read_binary_watch(num: i32) -> Vec<String> {
    todo!()
}
// backtracking bit_manipulation
#[test]
#[ignore]
fn test2_401() {
    assert_eq!(
        read_binary_watch(1),
        vec![
            String::from("1:00"),
            String::from("2:00"),
            String::from("4:00"),
            String::from("8:00"),
            String::from("0:01"),
            String::from("0:02"),
            String::from("0:04"),
            String::from("0:08"),
            String::from("0:16"),
            String::from("0:32")
        ]
    );
}