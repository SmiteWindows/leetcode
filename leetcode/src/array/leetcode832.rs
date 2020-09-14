// https://leetcode-cn.com/problems/flipping-an-image/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut a = a;
    let n = a.len();
    for i in a.iter_mut().take(n) {
        let mut l = 0;
        let mut r = n - 1;
        while l < r {
            i.swap(l, r);
            l += 1;
            r -= 1;
        }
        for j in i.iter_mut().take(n) {
            *j = 1 - *j;
        }
    }
    a
}
// array
#[test]
fn test1_832() {
    use leetcode_prelude::vec2;
    assert_eq!(
        flip_and_invert_image(vec2![[1, 1, 0], [1, 0, 1], [0, 0, 0]]),
        vec2![[1, 0, 0], [0, 1, 0], [1, 1, 1]]
    );
    assert_eq!(
        flip_and_invert_image(vec2![
            [1, 1, 0, 0],
            [1, 0, 0, 1],
            [0, 1, 1, 1],
            [1, 0, 1, 0]
        ]),
        vec2![[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 1], [1, 0, 1, 0]]
    );
}
