// https://leetcode.com/problems/open-the-lock/
pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_752() {
    assert_eq!(
        open_lock(
            vec![
                String::from("0201"),
                String::from("0101"),
                String::from("0102"),
                String::from("1212"),
                String::from("2002")
            ],
            String::from("0202")
        ),
        6
    );
    assert_eq!(
        open_lock(vec![String::from("8888")], String::from("0009")),
        1
    );
    assert_eq!(
        open_lock(
            vec![
                String::from("8887"),
                String::from("8889"),
                String::from("8878"),
                String::from("8898"),
                String::from("8788"),
                String::from("8988"),
                String::from("7888"),
                String::from("9888")
            ],
            String::from("8888")
        ),
        -1
    );
    assert_eq!(
        open_lock(vec![String::from("0000")], String::from("8888")),
        -1
    );
}
