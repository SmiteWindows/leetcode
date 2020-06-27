// https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/
pub fn average(salary: Vec<i32>) -> f64 {
    todo!()
}
// array sort
#[test]
#[ignore]
fn test1_1491() {
    assert_eq!(average(vec![4000, 3000, 1000, 2000]), 2500.00000);
    assert_eq!(average(vec![1000, 2000, 3000]), 2000.00000);
    assert_eq!(
        average(vec![6000, 5000, 4000, 3000, 2000, 1000]),
        3500.00000
    );
    assert_eq!(
        average(vec![8000, 9000, 2000, 3000, 6000, 1000]),
        4750.00000
    );
}
