// https://leetcode-cn.com/problems/group-the-people-given-the-group-size-they-belong-to/
// Runtime: 8 ms
// Memory Usage: 2.1 MB
type Person = (i32, usize);
pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
    let mut people: Vec<Person> = Vec::new();
    let mut res: Vec<Vec<i32>> = Vec::new();
    for (id, &group_size) in group_sizes.iter().enumerate() {
        people.push((group_size, id))
    }
    people.sort_unstable();
    let mut group: Vec<i32> = Vec::new();
    for p in people {
        group.push(p.1 as i32);
        if group.len() == p.0 as usize {
            res.push(group);
            group = Vec::new();
        }
    }
    res
}
// greedy
#[test]
fn test1_1282() {
    use leetcode_prelude::vec2;
    assert_eq!(
        group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
        vec2![[5], [0, 1, 2], [3, 4, 6]]
    );
    assert_eq!(
        group_the_people(vec![2, 1, 3, 3, 3, 2]),
        vec2![[1], [0, 5], [2, 3, 4]]
    );
}
