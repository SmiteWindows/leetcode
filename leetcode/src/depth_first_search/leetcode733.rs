// https://leetcode-cn.com/problems/flood-fill/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    let mut image = image;
    let sr = sr as usize;
    let sc = sc as usize;
    let color = image[sr][sc];
    if color != new_color {
        dfs(&mut image, sr, sc, color, new_color);
    }
    image
}

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
// depth_first_search
#[test]
fn test1_733() {
    use leetcode_prelude::vec2;
    assert_eq!(
        flood_fill(vec2![[1, 1, 1], [1, 1, 0], [1, 0, 1]], 1, 1, 2),
        vec2![[2, 2, 2], [2, 2, 0], [2, 0, 1]]
    );
}
