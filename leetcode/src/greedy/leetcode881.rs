// https://leetcode.com/problems/boats-to-save-people/
// Runtime: 20 ms
// Memory Usage: 2.3 MB
pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let mut people = people;
    let n = people.len();
    people.sort();
    people.reverse();
    let mut i = 0;
    let mut j = n - 1;
    let mut res = 0;
    while i <= j {
        res += 1;
        if people[i] + people[j] <= limit {
            j -= 1;
        }
        i += 1;
    }
    res
}
// two_pointers greedy
#[test]
fn test2_881() {
    assert_eq!(num_rescue_boats(vec![1, 2], 3), 1);
    assert_eq!(num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
    assert_eq!(num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
}
