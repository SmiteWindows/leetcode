// https://leetcode-cn.com/problems/squares-of-a-sorted-array/
pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    // let n = a.len();
    // let mut j = 0;
    // while j < n && a[j] < 0 {
    //     j += 1;
    // }
    // let mut i = j - 1;
    // let mut res = vec![0; n];
    // let mut t = 0;
    // while i >= 0 && j < n {
    //     if a[i] * a[i] < a[j] * a[j] {
    //         res[t] = a[i] * a[i];
    //         t += 1;
    //         i -= 1;
    //     } else {
    //         res[t] = a[j] * a[j];
    //         t += 1;
    //         j += 1;
    //     }
    // }
    // while i >= 0 {
    //     res[t] = a[i] * a[i];
    //     t += 1;
    //     i -= 1;
    // }
    // while j < n {
    //     res[t] = a[j] * a[j];
    //     t += 1;
    //     j += 1;
    // }
    // res
    todo!()
}
// two_pointers array
#[test]
#[ignore]
fn test1_977() {
    assert_eq!(
        sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
    assert_eq!(
        sorted_squares(vec![-7, -3, 2, 3, 11]),
        vec![4, 9, 9, 49, 121]
    );
}
