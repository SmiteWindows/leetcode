// https://leetcode.com/problems/exclusive-time-of-functions/
pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    todo!()
}
// stack
#[test]
#[ignore]
fn test1_636() {
    assert_eq!(
        exclusive_time(
            2,
            vec![
                String::from("0:start:0"),
                String::from("1:start:2"),
                String::from("1:end:5"),
                String::from("0:end:6")
            ]
        ),
        vec![3, 4]
    );
}
