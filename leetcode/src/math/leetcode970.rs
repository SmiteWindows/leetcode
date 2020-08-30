// https://leetcode-cn.com/problems/powerful-integers/
// Runtime: 4 ms
// Memory Usage: 3 MB
pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
    let mut set = vec![false; bound as usize + 1];
    let mut i = 0;
    while x.pow(i) < bound {
        let mut j = 0;
        while y.pow(j) < bound {
            let sum = x.pow(i) + y.pow(j);
            if sum <= bound {
                set[sum as usize] = true;
            }
            j += 1;
            if y == 1 {
                break;
            }
        }
        i += 1;
        if x == 1 {
            break;
        }
    }
    let mut res = vec![];
    for i in 0..=bound {
        if set[i as usize] {
            res.push(i);
        }
    }
    res
}
// math hash_table
#[test]
fn test1_970() {
    assert_eq!(powerful_integers(2, 3, 10), vec![2, 3, 4, 5, 7, 9, 10]);
    assert_eq!(powerful_integers(3, 5, 15), vec![2, 4, 6, 8, 10, 14]);
}
