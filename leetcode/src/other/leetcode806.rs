// https://leetcode-cn.com/problems/number-of-lines-to-write-string/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    let mut lines = 0;
    let mut start = 0;
    for b in s.bytes() {
        let w = widths[(b - b'a') as usize];
        if start + w > 100 {
            lines += 1;
            start = w;
        } else {
            start += w;
        }
    }
    vec![lines + 1, start]
}
#[test]
fn test806() {
    assert_eq!(
        number_of_lines(
            vec![
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            String::from("abcdefghijklmnopqrstuvwxyz")
        ),
        vec![3, 60]
    );
    assert_eq!(
        number_of_lines(
            vec![
                4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            String::from("bbbcccdddaaa")
        ),
        vec![2, 4]
    );
}
