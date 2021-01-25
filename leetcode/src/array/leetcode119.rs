// https://leetcode-cn.com/problems/pascals-triangle-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn get_row(row_index: i32) -> Vec<i32> {
    let row_index = row_index as usize;
    if row_index == 0 {
        return vec![1];
    }
    if row_index == 1 {
        return vec![1, 1];
    }
    let mut row = vec![1, 1];
    for i in 2..=row_index {
        let mut temp = vec![1; row_index as usize + 1];
        for j in 1..i {
            temp[j] = row[j - 1] + row[j]
        }
        row = temp
    }
    row
}
// array
#[test]
fn test1_119() {
    assert_eq!(get_row(3), vec![1, 3, 3, 1],);
}
