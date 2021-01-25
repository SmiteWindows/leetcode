// https://leetcode-cn.com/problems/number-of-burgers-with-no-waste-of-ingredients/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
    if tomato_slices - 2 * cheese_slices < 0 {
        return vec![];
    }
    if (tomato_slices - 2 * cheese_slices) % 2 != 0 {
        return vec![];
    }
    if cheese_slices * 4 - tomato_slices < 0 {
        return vec![];
    }
    if (cheese_slices * 4 - tomato_slices) % 2 != 0 {
        return vec![];
    }
    let x = (tomato_slices - 2 * cheese_slices) / 2;
    let y = (cheese_slices * 4 - tomato_slices) / 2;
    vec![x, y]
}
// math greedy
#[test]
fn test1_1276() {
    assert_eq!(num_of_burgers(16, 7), vec![1, 6]);
    assert_eq!(num_of_burgers(17, 4), vec![]);
    assert_eq!(num_of_burgers(4, 17), vec![]);
    assert_eq!(num_of_burgers(0, 0), vec![0, 0]);
    assert_eq!(num_of_burgers(2, 1), vec![0, 1]);
}
