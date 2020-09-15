// https://leetcode-cn.com/problems/filling-bookcase-shelves/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let n = books.len();
    let mut dp = vec![0; n + 1];
    for i in 0..n {
        let mut width = books[i][0];
        let mut height = books[i][1];
        dp[i + 1] = height + dp[i];
        for j in (0..i).rev() {
            width += books[j][0];
            if width <= shelf_width {
                height = height.max(books[j][1]);
                dp[i + 1] = dp[i + 1].min(height + dp[j]);
            }
        }
    }
    dp[n]
}
// dynamic_programming
#[test]
fn test1_1105() {
    use leetcode_prelude::vec2;
    assert_eq!(
        min_height_shelves(
            vec2![[1, 1], [2, 3], [2, 3], [1, 1], [1, 1], [1, 1], [1, 2]],
            4
        ),
        6
    );
}
