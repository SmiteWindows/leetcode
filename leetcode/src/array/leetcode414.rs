// https://leetcode-cn.com/problems/third-maximum-number/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn third_max(nums: Vec<i32>) -> i32 {
    let mut max1 = nums[0];
    let mut m2 = None;
    let mut m3 = None;
    for x in nums {
        match x.cmp(&max1) {
            Greater => {
                m3 = m2;
                m2 = Some(max1);
                max1 = x;
            }
            Less => {
                if let Some(max2) = m2 {
                    match x.cmp(&max2) {
                        Greater => {
                            m3 = m2;
                            m2 = Some(x);
                        }
                        Less => {
                            if let Some(max3) = m3 {
                                if x > max3 {
                                    m3 = Some(x);
                                }
                            } else {
                                m3 = Some(x);
                            }
                        }
                        Equal => {}
                    }
                } else {
                    m2 = Some(x);
                }
            }
            Equal => {}
        }
    }
    if let Some(max3) = m3 {
        max3
    } else {
        max1
    }
}
// array
#[test]
fn test1_414() {
    assert_eq!(third_max(vec![3, 2, 1]), 1);
    assert_eq!(third_max(vec![1, 2]), 2);
    assert_eq!(third_max(vec![2, 2, 3, 1]), 1);
}
