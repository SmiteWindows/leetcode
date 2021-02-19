// https://leetcode-cn.com/problems/lemonade-change/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut b5 = 0;
    let mut b10 = 0;
    for b in bills {
        match b {
            5 => {
                b5 += 1;
            }
            10 => {
                if b5 > 0 {
                    b10 += 1;
                    b5 -= 1;
                } else {
                    return false;
                }
            }
            20 => {
                if b10 > 0 && b5 > 0 {
                    b10 -= 1;
                    b5 -= 1;
                } else if b5 >= 3 {
                    b5 -= 3;
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}
// greedy
#[test]
fn test1_860() {
    assert_eq!(lemonade_change(vec![5, 5, 5, 10, 20]), true);
    assert_eq!(lemonade_change(vec![5, 5, 10]), true);
    assert_eq!(lemonade_change(vec![10, 10]), false);
    assert_eq!(lemonade_change(vec![5, 5, 10, 10, 20]), false);
}
