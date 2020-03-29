// https://leetcode.com/problems/two-sum/
/// Runtime: 28 ms
/// Memory Usage: 2.2 MB
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut rev = vec![-1i32; 2];
    for i in 0..nums.len() {
        let a = target - nums[i];
        for k in i + 1..nums.len() {
            if a == nums[k] {
                rev[0] = i as i32;
                rev[1] = k as i32;
                return rev;
            }
        }
    }
    rev
}
#[test]
fn test1_1(){
    let v1 = vec![2,7,11,15];
    let a = two_sum(v1, 9);
    assert_eq!(a,vec![0,1]);
}