// https://leetcode-cn.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn average(salary: Vec<i32>) -> f64 {
    let n = salary.len();
    let min = *salary.iter().min().unwrap();
    let max = *salary.iter().max().unwrap();
    let sum = salary
        .into_iter()
        .filter(|&x| x > min && x < max)
        .sum::<i32>();
    sum as f64 / (n - 2) as f64
}
// array sort
#[test]
fn test2_1491() {
    use leetcode_prelude::assert_approx_eq;
    assert_approx_eq!(average(vec![4000, 3000, 1000, 2000]), 2500.00000);
    assert_approx_eq!(average(vec![1000, 2000, 3000]), 2000.00000);
    assert_approx_eq!(
        average(vec![6000, 5000, 4000, 3000, 2000, 1000]),
        3500.00000
    );
    assert_approx_eq!(
        average(vec![8000, 9000, 2000, 3000, 6000, 1000]),
        4750.00000
    );
}
