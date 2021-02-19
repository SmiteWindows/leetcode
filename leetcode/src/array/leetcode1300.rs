// https://leetcode-cn.com/problems/sum-of-mutated-array-closest-to-target/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn find_best_value(mut arr: Vec<i32>, mut target: i32) -> i32 {
    arr.sort_unstable();
    let n = arr.len();
    let mut i = 0;
    while i < n && target > arr[i] * (n - i) as i32 {
        target -= arr[i];
        i += 1;
    }
    if i == n {
        return arr[n - 1];
    }
    let res = target / (n - i) as i32;
    if (res + 1) * (n - i) as i32 - target < target - res * (n - i) as i32 {
        res + 1
    } else {
        res
    }
}
// binary_search array
#[test]
fn test2_1300() {
    assert_eq!(find_best_value(vec![4, 9, 3], 10), 3);
    assert_eq!(find_best_value(vec![2, 3, 5], 10), 5);
    assert_eq!(
        find_best_value(vec![60864, 25176, 27249, 21296, 20204], 56803),
        11361
    );
}
