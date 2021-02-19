// https://leetcode-cn.com/problems/optimal-division/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn optimal_division(nums: Vec<i32>) -> String {
    let n = nums.len();
    if n == 1 {
        format!("{}", nums[0])
    } else if n == 2 {
        format!("{}/{}", nums[0], nums[1])
    } else {
        let mid = (1..n)
            .map(|i| nums[i].to_string())
            .collect::<Vec<String>>()
            .join("/");
        format!("{}/({})", nums[0], mid)
    }
}
// math string
#[test]
fn test2_553() {
    assert_eq!(
        optimal_division(vec![1000, 100, 10, 2]),
        "1000/(100/10/2)".to_string()
    );
}
