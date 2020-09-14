//https://leetcode-cn.com/problems/add-to-array-form-of-integer/
// Runtime: 12 ms
// Memory Usage: 2.2 MB
pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
    let mut a = a;
    let mut k = k;
    let mut i = a.len() - 1;
    while k > 0 {
        let sum = a[i] + k;
        a[i] = sum % 10;
        k = sum / 10;
        if i > 0 {
            i -= 1;
        } else {
            a.insert(0, 0);
        }
    }
    if a.len() > 1 && a[0] == 0 {
        a.remove(0);
    }
    a
}
// array
#[test]
fn test1_989() {
    assert_eq!(add_to_array_form(vec![1, 2, 0, 0], 34), vec![1, 2, 3, 4]);
    assert_eq!(add_to_array_form(vec![2, 7, 4], 181), vec![4, 5, 5]);
    assert_eq!(add_to_array_form(vec![2, 1, 5], 806), vec![1, 0, 2, 1]);
    assert_eq!(
        add_to_array_form(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1),
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}
