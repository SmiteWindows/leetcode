// https://leetcode.com/problems/sort-array-by-parity/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
    let mut a = a;
    let mut l = 0;
    let mut r = a.len() - 1;
    while l < r {
        while a[l] % 2 == 0 && l < r {
            l += 1;
        }

        while a[r] % 2 == 1 && l < r {
            r -= 1;
        }

        if l < r {
            a.swap(l, r);
        }
    }
    a
}
// array
#[test]
fn test1_905() {
    assert_eq!(sort_array_by_parity(vec![3, 1, 2, 4]), vec![4, 2, 1, 3]);
}
