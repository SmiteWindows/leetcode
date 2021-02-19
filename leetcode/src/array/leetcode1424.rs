// https://leetcode-cn.com/problems/diagonal-traverse-ii/
// Runtime: 48 ms
// Memory Usage: 10.4 MB
pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums.len();
    let mut rows = Vec::new();
    for (i, num) in nums.iter().enumerate().take(n) {
        for (j, &v) in num.iter().enumerate() {
            let k = i + j;
            if rows.len() == k {
                rows.push(vec![]);
            }
            rows[k].push(v);
        }
    }
    rows.into_iter().flat_map(|q| q.into_iter().rev()).collect()
}
// array sort
#[test]
fn test1_1424() {
    use leetcode_prelude::vec2;
    assert_eq!(
        find_diagonal_order(vec2![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
        vec![1, 4, 2, 7, 5, 3, 8, 6, 9]
    );
    assert_eq!(
        find_diagonal_order(vec2![
            [1, 2, 3, 4, 5],
            [6, 7],
            [8],
            [9, 10, 11],
            [12, 13, 14, 15, 16]
        ]),
        vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16]
    );
    assert_eq!(
        find_diagonal_order(vec2![[1, 2, 3], [4], [5, 6, 7], [8], [9, 10, 11]]),
        vec![1, 4, 2, 5, 3, 8, 6, 9, 7, 10, 11]
    );
    assert_eq!(
        find_diagonal_order(vec2![[1, 2, 3, 4, 5, 6]]),
        vec![1, 2, 3, 4, 5, 6]
    );
}
