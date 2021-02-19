// https://leetcode-cn.com/problems/bulb-switcher-iii/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn num_times_all_blue(light: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut max = usize::MIN;
    for (i, &li) in light.iter().enumerate() {
        max = max.max((li - 1) as usize);
        if max == i {
            res += 1;
        }
    }
    res
}
// array
#[test]
fn test1_1375() {
    assert_eq!(num_times_all_blue(vec![2, 1, 3, 5, 4]), 3);
    assert_eq!(num_times_all_blue(vec![3, 2, 4, 1, 5]), 2);
    assert_eq!(num_times_all_blue(vec![4, 1, 2, 3]), 1);
    assert_eq!(num_times_all_blue(vec![2, 1, 4, 3, 6, 5]), 3);
    assert_eq!(num_times_all_blue(vec![1, 2, 3, 4, 5, 6]), 6);
}
