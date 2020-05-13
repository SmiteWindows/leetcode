// https://leetcode.com/problems/flood-fill/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    fn dfs(image: &mut [Vec<i32>], r: usize, c: usize, color: i32, new_color: i32) {
        if image[r][c] == color {
            image[r][c] = new_color;
            if r >= 1 {
                dfs(image, r - 1, c, color, new_color);
            }
            if c >= 1 {
                dfs(image, r, c - 1, color, new_color);
            }
            if r + 1 < image.len() {
                dfs(image, r + 1, c, color, new_color);
            }
            if c + 1 < image[0].len() {
                dfs(image, r, c + 1, color, new_color);
            }
        }
    }

    let mut image = image;
    let sr = sr as usize;
    let sc = sc as usize;
    let color = image[sr][sc];
    if color != new_color {
        dfs(&mut image, sr, sc, color, new_color);
    }
    image
}
// depth_first_search
#[test]
fn test1_733() {
    assert_eq!(
        flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
        vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
    );
}
