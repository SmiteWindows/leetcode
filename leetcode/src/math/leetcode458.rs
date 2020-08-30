// https://leetcode-cn.com/problems/poor-pigs/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
    let mut pigs = 0;
    let t = minutes_to_test / minutes_to_die + 1;
    while t.pow(pigs) < buckets {
        pigs += 1;
    }
    pigs as i32    
}
// math
#[test]
fn test1_458(){
    assert_eq!(poor_pigs(1000, 15, 60),5);
}