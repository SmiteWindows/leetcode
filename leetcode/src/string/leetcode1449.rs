// https://leetcode-cn.com/problems/form-largest-integer-with-digits-that-add-up-to-target/
pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
    todo!()
}
// string dynamic_programming
#[test]
#[ignore]
fn test1_1449() {
    assert_eq!(
        largest_number(vec![4, 3, 2, 5, 6, 7, 2, 5, 5], 9),
        String::from("7772")
    );
    assert_eq!(
        largest_number(vec![7, 6, 5, 5, 5, 6, 8, 7, 8], 12),
        String::from("85")
    );
    assert_eq!(
        largest_number(vec![2, 4, 6, 2, 4, 6, 4, 4, 4], 5),
        String::from("0")
    );
    assert_eq!(
        largest_number(vec![6, 10, 15, 40, 40, 40, 40, 40, 40], 47),
        String::from("32211")
    );
}
