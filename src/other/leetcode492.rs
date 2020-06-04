// https://leetcode.com/problems/construct-the-rectangle/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn construct_rectangle(area: i32) -> Vec<i32> {
    let mut max = (area as f64).sqrt().floor() as i32;
    while area % max != 0 {
        max -= 1;
    }
    vec![area / max, max]
}
#[test]
fn test492() {
    assert_eq!(construct_rectangle(4), vec![2, 2]);
}
