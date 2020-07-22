// https://leetcode.com/problems/distribute-candies-to-people/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
    let mut candies = candies;
    let mut i = 0;
    let n = num_people as usize;
    let mut res: Vec<i32> = vec![0; n];
    let mut gift = 0;
    while candies > 0 {
        gift = candies.min(gift + 1);
        res[i] += gift;
        candies -= gift;
        i = (i + 1) % n;
    }
    res
}
// math
#[test]
fn test1_1103() {
    assert_eq!(distribute_candies(7, 4), vec![1, 2, 3, 1]);
    assert_eq!(distribute_candies(10, 3), vec![5, 2, 3]);
}
