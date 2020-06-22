// https://leetcode.com/problems/corporate-flight-bookings/
// Runtime: 28 ms
// Memory Usage: 3.7 MB
pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut res = vec![0; n];
    for booking in bookings {
        let start = (booking[0] - 1) as usize;
        let end = booking[1] as usize;
        res[start] += booking[2];
        if end < n {
            res[end] -= booking[2];
        }
    }
    let mut prev = 0;
    for i in 0..n {
        prev += res[i];
        res[i] = prev;
    }
    res
}
// math array
#[test]
fn test1_1109() {
    assert_eq!(
        corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5),
        vec![10, 55, 45, 25, 25]
    );
}
