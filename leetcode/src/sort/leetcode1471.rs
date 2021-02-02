// https://leetcode-cn.com/problems/the-k-strongest-values-in-an-array/
// Runtime: 60 ms
// Memory Usage: 3 MB
pub fn get_strongest(mut arr: Vec<i32>, mut k: i32) -> Vec<i32> {
    arr.sort_unstable();
    let n = arr.len();
    let median = arr[(n - 1) / 2];
    let mut l = 0;
    let mut r = n - 1;
    let mut res = Vec::new();
    while k > 0 {
        if (arr[l] - median).abs() <= (arr[r] - median).abs() {
            res.push(arr[r]);
            r -= 1;
        } else {
            res.push(arr[l]);
            l += 1;
        }
        k -= 1;
    }
    res
}
// array sort
#[test]
fn test2_1471() {
    assert_eq!(get_strongest(vec![1, 2, 3, 4, 5], 2), vec![5, 1]);
    assert_eq!(get_strongest(vec![1, 1, 3, 5, 5], 2), vec![5, 5]);
    assert_eq!(
        get_strongest(vec![6, 7, 11, 7, 6, 8], 5),
        vec![11, 8, 6, 6, 7]
    );
    assert_eq!(get_strongest(vec![6, -3, 7, 2, 11], 3), vec![-3, 11, 2]);
    assert_eq!(get_strongest(vec![-7, 22, 17, 3], 2), vec![22, 17]);
}
