// https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/
pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1444() {
    assert_eq!(
        ways(
            vec![
                String::from("A.."),
                String::from("AAA"),
                String::from("...")
            ],
            3
        ),
        3
    );
    assert_eq!(
        ways(
            vec![
                String::from("A.."),
                String::from("AA."),
                String::from("...")
            ],
            3
        ),
        1
    );
    assert_eq!(
        ways(
            vec![
                String::from("A.."),
                String::from("A.."),
                String::from("...")
            ],
            1
        ),
        1
    );
}
