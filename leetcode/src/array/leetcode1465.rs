// https://leetcode-cn.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/
// Runtime: 12 ms
// Memory Usage: 4.1 MB
pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
    let mut horizontal_cuts = horizontal_cuts;
    let mut vertical_cuts = vertical_cuts;
    horizontal_cuts.push(0);
    horizontal_cuts.push(h);
    horizontal_cuts.sort_unstable();
    vertical_cuts.push(0);
    vertical_cuts.push(w);
    vertical_cuts.sort_unstable();
    let h = horizontal_cuts
        .windows(2)
        .map(|v| v[1] - v[0])
        .max()
        .unwrap();
    let w = vertical_cuts.windows(2).map(|v| v[1] - v[0]).max().unwrap();
    ((h as i64 * w as i64) % 1_000_000_007) as i32
}
// array
#[test]
fn test1_1465() {
    assert_eq!(max_area(5, 4, vec![1, 2, 4], vec![1, 3]), 4);
    assert_eq!(max_area(5, 4, vec![3, 1], vec![1]), 6);
    assert_eq!(max_area(5, 4, vec![3], vec![3]), 9);
}
