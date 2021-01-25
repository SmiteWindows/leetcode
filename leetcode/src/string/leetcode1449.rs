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
        "7772".to_string()
    );
    assert_eq!(
        largest_number(vec![7, 6, 5, 5, 5, 6, 8, 7, 8], 12),
        "85".to_string()
    );
    assert_eq!(
        largest_number(vec![2, 4, 6, 2, 4, 6, 4, 4, 4], 5),
        "0".to_string()
    );
    assert_eq!(
        largest_number(vec![6, 10, 15, 40, 40, 40, 40, 40, 40], 47),
        "32211".to_string()
    );
}
