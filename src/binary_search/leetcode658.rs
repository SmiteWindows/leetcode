// https://leetcode.com/problems/find-k-closest-elements/
// Runtime: 8 ms
// Memory Usage: 2.3 MB
pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let n = arr.len();
    let k = k as usize;
    let mut l = 0;
    let mut r = n - k;
    while l < r {
        let m = l + (r - l) / 2;
        if x - arr[m] > arr[m + k] - x {
            l = m + 1;
        } else {
            r = m;
        }
    }
    arr[l..l + k].to_vec()
}
// binary_search
#[test]
fn test1_658() {
    assert_eq!(
        find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1),
        vec![1, 2, 3, 4]
    );
}
