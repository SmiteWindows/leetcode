// https://leetcode.com/problems/duplicate-zeros/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let n = arr.len();
    let m = arr.iter().filter(|&x| *x == 0).count();
    let mut i = n - 1;
    let mut j = m + n - 1;
    while i > 0 {
        if arr[i] != 0 {
            if j < n {
                arr[j] = arr[i];
            }
            i -= 1;
            j -= 1;
        } else {
            if j < n {
                arr[j] = 0;
            }
            i -= 1;
            j -= 1;
            if j < n {
                arr[j] = 0;
            }
            j -= 1;
        }
    }
    if arr[i] != 0 {
        arr[j] = arr[i];
    } else {
        arr[j] = 0;
        j -= 1;
        arr[j] = 0;
    }
}
// array
#[test]
fn test1_1089() {
    let mut arr1 = vec![1, 0, 2, 3, 0, 4, 5, 0];
    duplicate_zeros(&mut arr1);
    assert_eq!(arr1, vec![1, 0, 0, 2, 3, 0, 0, 4]);
    let mut arr2 = vec![1, 2, 3];
    duplicate_zeros(&mut arr2);
    assert_eq!(arr2, vec![1, 2, 3]);
}
