// https://leetcode.com/problems/subrectangle-queries/
// Runtime: 8 ms
// Memory Usage: 3.2 MB
struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>,
    n: usize,
    m: usize,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        let n = rectangle.len();
        let m = rectangle[0].len();
        Self { rectangle, n, m }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        let r1 = row1 as usize;
        let c1 = col1 as usize;
        let r2 = row2 as usize;
        let c2 = col2 as usize;
        for i in r1..=r2 {
            for j in c1..=c2 {
                self.rectangle[i][j] = new_value;
            }
        }
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.rectangle[row as usize][col as usize]
    }
}
/**
 * Your SubrectangleQueries object will be instantiated and called as such:
 * let obj = SubrectangleQueries::new(rectangle);
 * obj.update_subrectangle(row1, col1, row2, col2, newValue);
 * let ret_2: i32 = obj.get_value(row, col);
 */
// array
#[test]
fn test1_1476() {
    let rectangle = vec![vec![1, 2, 1], vec![4, 3, 4], vec![3, 2, 1], vec![1, 1, 1]];
    let mut obj = SubrectangleQueries::new(rectangle);
    assert_eq!(obj.get_value(0, 2), 1);
    obj.update_subrectangle(0, 0, 3, 2, 5);
    assert_eq!(obj.get_value(0, 2), 5);
    assert_eq!(obj.get_value(3, 1), 5);
    obj.update_subrectangle(3, 0, 3, 2, 10);
    assert_eq!(obj.get_value(3, 1), 10);
    assert_eq!(obj.get_value(0, 2), 5);

    let rectangle = vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3]];
    let mut obj = SubrectangleQueries::new(rectangle);
    assert_eq!(obj.get_value(0, 0), 1);
    obj.update_subrectangle(0, 0, 2, 2, 100);
    assert_eq!(obj.get_value(0, 0), 100);
    assert_eq!(obj.get_value(2, 2), 100);
    obj.update_subrectangle(1, 1, 2, 2, 20);
    assert_eq!(obj.get_value(2, 2), 20);
}
