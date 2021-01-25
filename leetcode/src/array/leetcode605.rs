// https://leetcode-cn.com/problems/can-place-flowers/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
    let m = flowerbed.len();
    let mut sum = 0;
    for i in 0..m {
        if flowerbed[i] == 0 {
            if i == 0 || flowerbed[i - 1] == 0 {
                sum += 1;
                flowerbed[i] = 1;
            }
        } else if i > 0 && flowerbed[i - 1] == 1 {
            sum -= 1;
        }
    }
    sum >= n
}
// array
#[test]
fn test1_605() {
    assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    assert_eq!(can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
}
