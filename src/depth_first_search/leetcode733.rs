// https://leetcode.com/problems/flood-fill/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::collections::VecDeque;
struct Pixel {
    i: usize,
    j: usize,
}
pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    let mut image = image;
    let n = image.len();
    let m = image[0].len();
    let sr = sr as usize;
    let sc = sc as usize;
    if sr >= n {
        return image;
    }
    if sc >= m {
        return image;
    }
    let c = image[sr][sc];
    if c == new_color {
        return image;
    }
    let mut queue = VecDeque::new();
    queue.push_back(Pixel { i: sr, j: sc });
    let di = vec![0, 0, -1, 1];
    let dj = vec![-1, 1, 0, 0];
    while let Some(pixel) = queue.pop_front() {
        let i = pixel.i;
        let j = pixel.j;
        image[i][j] = new_color;
        for k in 0..4 {
            let i = i as i32 + di[k];
            let j = j as i32 + dj[k];
            let i = i as usize;
            let j = j as usize;
            if i >= n {
                continue;
            }
            if j >= m {
                continue;
            }
            if image[i][j] != c {
                continue;
            }
            queue.push_back(Pixel { i, j })
        }
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
