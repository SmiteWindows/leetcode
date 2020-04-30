// https://leetcode.com/problems/utf-8-validation/
pub fn valid_utf8(data: Vec<i32>) -> bool {
    // let mut p = 0;
    // let n = data.len();
    // while p < n {
    //     let v = data[p];
    //     let mut cnt = 0;
    //     if (v & 0b10000000) == 0 {
    //         cnt = 0;
    //     } else if (v & 0b11100000) == 0b11000000 {
    //         cnt = 1;
    //     } else if (v & 0b11110000) == 0b11100000 {
    //         cnt = 2;
    //     } else if (v & 0b11111000) == 0b11110000 {
    //         cnt = 3;
    //     } else {
    //         return false;
    //     }
    //     p += 1;
    //     while p < n && cnt > 0 {
    //         if (data[p] & 0b11000000) != 0b10000000 {
    //             break;
    //         }
    //         p += 1;
    //         cnt -= 1;
    //     }
    //     if cnt > 0 {
    //         return false;
    //     }
    // }
    // true
    todo!()
}
// bit_manipulation
#[test]
#[ignore]
fn test1_393() {
    assert_eq!(valid_utf8(vec![197, 130, 1]), true);
    assert_eq!(valid_utf8(vec![235, 140, 4]), false);
}
