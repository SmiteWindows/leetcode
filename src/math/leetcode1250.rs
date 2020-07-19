// https://leetcode.com/problems/check-if-it-is-a-good-array/
// Runtime: 12 ms
// Memory Usage: 3.3 MB
pub fn is_good_array(nums: Vec<i32>) -> bool {
    let mut res = nums[0];
    let n = nums.len();
    for i in 0..n {
        res = gcd(res, nums[i]);
        if res == 1 {
            return true;
        }
    }
    false
}

fn gcd(mut m: i32, mut n: i32) -> i32 {
    while m != 0 {
        let temp = m;
        m = n % temp;
        n = temp;
    }
    n.abs()
}
// math
#[test]
fn test1_1250() {
    assert_eq!(is_good_array(vec![12, 5, 7, 23]), true);
    assert_eq!(is_good_array(vec![29, 6, 10]), true);
    assert_eq!(is_good_array(vec![3, 6]), false);
}
