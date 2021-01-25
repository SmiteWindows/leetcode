// https://leetcode-cn.com/problems/largest-number/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn largest_number(nums: Vec<i32>) -> String {
    let mut v = nums.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| format!("{}{}", b, a).cmp(&format!("{}{}", a, b)));
    let mut res: String = "".to_string();
    if v[0] == "0" {
        return "0".to_string();
    }
    for s in v {
        res += &s;
    }
    res
}
// sort
#[test]
fn test1_179() {
    assert_eq!(largest_number(vec![10, 2]), "210".to_string());
    assert_eq!(largest_number(vec![3, 30, 34, 5, 9]), "9534330".to_string());
}
