// https://leetcode.com/problems/swap-adjacent-in-lr-string/
pub fn can_transform(start: String, end: String) -> bool {
    let mut iter_a = start.chars();
    let mut iter_b = end.chars();
    let mut count_a = 0;
    let mut count_b = 0;
    while let (Some(mut a), Some(mut b)) = (iter_a.next(), iter_b.next()) {
        while a == 'X' {
            count_a += 1;
            a = if let Some(c) = iter_a.next() { c } else { ' ' };
        }
        while b == 'X' {
            count_b += 1;
            b = if let Some(c) = iter_b.next() { c } else { ' ' };
        }
        if a != b {
            dbg!(count_a);
            dbg!(count_b);
            return false;
        }
        if a == 'R' && b == 'R' && count_a > count_b {
            dbg!(count_a);
            dbg!(count_b);
            return false;
        }
        if a == 'L' && b == 'L' && count_a < count_b {
            dbg!(count_a);
            dbg!(count_b);
            return false;
        }
    }
    dbg!(count_a);
    dbg!(count_b);
    true
}
// brainteaser
#[test]
#[ignore]
fn test1_777() {
    // assert_eq!(
    //     can_transform(String::from("RXR"), String::from("XXR")),
    //     false,
    // );
    assert_eq!(
        can_transform(String::from("RXXLRXRXL"), String::from("XRLXXRRLX")),
        true,
    );
}
