// https://leetcode-cn.com/problems/largest-rectangle-in-histogram/
// Runtime: 0 ms
// Memory Usage: 2.4 MB
pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
    let mut stack: Vec<(i32, i32)> = vec![(0, 0)];
    heights.push(0);
    let n = heights.len();
    let mut res = 0;
    for (i, &height) in heights.iter().enumerate().take(n) {
        let x1 = (i + 1) as i32;
        let y1 = height;
        let mut x3 = x1;
        while let Some(&(x2, y2)) = stack.last() {
            if y2 > y1 {
                stack.pop();
                res = res.max((x1 - x2) * y2);
                x3 = x2;
            } else {
                stack.push((x3, y1));
                break;
            }
        }
    }
    res
}
// stack array
#[test]
fn test2_84() {
    assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
}
