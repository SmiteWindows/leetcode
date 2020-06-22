// https://leetcode.com/problems/arithmetic-slices/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
    let mut slice = vec![];
    let mut res = 0;
    for x in a {
        if slice.len() < 2 {
            slice.push(x);
        } else {
            let y = slice.pop().unwrap();
            let z = slice.pop().unwrap();
            if y - z == x - y {
                slice.push(z);
                slice.push(y);
                slice.push(x);
                res += slice.len() - 2;
            } else {
                slice.clear();
                slice.push(y);
                slice.push(x);
            }
        }
    }
    res as i32
}
// math dynamic_programming
#[test]
fn test2_413() {
    assert_eq!(number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
}
