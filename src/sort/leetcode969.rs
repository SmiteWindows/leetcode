// https://leetcode.com/problems/pancake-sorting/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn pancake_sort(a: Vec<i32>) -> Vec<i32> {
    let mut a = a;
    let n = a.len();
    let mut res = vec![];
    for i in 0..n {
        let (_, j) = (0..n - i).fold(
            (a[0], 0),
            |acc, j| {
                if a[j] > acc.0 {
                    (a[j], j)
                } else {
                    acc
                }
            },
        );
        a[0..=j].reverse();
        a[0..n - i].reverse();
        res.push(j + 1);
        res.push(n - i);
    }
    res.iter().map(|&x| x as i32).collect()
}
// sort array
#[test]
fn test1_969() {
    assert_eq!(pancake_sort(vec![3, 2, 4, 1]), vec![4, 2, 4, 3]);
    assert_eq!(pancake_sort(vec![1, 2, 3]), vec![]);
}
