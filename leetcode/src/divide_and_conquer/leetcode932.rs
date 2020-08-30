// https://leetcode-cn.com/problems/beautiful-array/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn beautiful_array(n: i32) -> Vec<i32> {
    if n == 1 {
        vec![1]
    } else {
        let left = beautiful_array(n / 2);
        let right = beautiful_array((n + 1) / 2);
        left.into_iter()
            .map(|x| x * 2)
            .chain(right.into_iter().map(|x| x * 2 - 1))
            .collect()
    }
}
// divide_and_conquer
#[test]
fn test1_932() {
    assert_eq!(beautiful_array(5), vec![4, 2, 3, 5, 1]);
    assert_eq!(beautiful_array(4), vec![4, 2, 3, 1]);
}
