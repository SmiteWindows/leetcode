// https://leetcode.com/problems/ones-and-zeroes/
pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_474() {
    assert_eq!(
        find_max_form(
            vec![
                String::from("10"),
                String::from("0001"),
                String::from("111001"),
                String::from("1"),
                String::from("0")
            ],
            5,
            3
        ),
        4
    );
    assert_eq!(
        find_max_form(
            vec![String::from("10"), String::from("0"), String::from("1")],
            1,
            1
        ),
        2
    );
}
