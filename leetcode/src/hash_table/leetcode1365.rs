// https://leetcode-cn.com/problems/how-many-numbers-are-smaller-than-the-current-number/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut count = vec![0; 101];
    for &x in &nums {
        count[x as usize] += 1;
    }
    for i in 0..100 {
        count[i + 1] += count[i]
    }
    let mut res = vec![];
    for &x in &nums {
        let v = if x == 0 { 0 } else { count[(x - 1) as usize] };
        res.push(v)
    }
    res
}
// hash_table array
#[test]
fn test1_1365() {
    assert_eq!(
        smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
        vec![4, 0, 1, 1, 3]
    );
    assert_eq!(
        smaller_numbers_than_current(vec![6, 5, 4, 8]),
        vec![2, 1, 0, 3]
    );
    assert_eq!(
        smaller_numbers_than_current(vec![7, 7, 7, 7]),
        vec![0, 0, 0, 0]
    );
}
