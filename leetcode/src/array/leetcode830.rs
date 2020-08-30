// https://leetcode-cn.com/problems/positions-of-large-groups/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    let mut prev: Option<Group> = None;
    let mut groups = vec![];
    for (i, c) in s.char_indices() {
        if let Some(prev_group) = prev {
            if prev_group.c == c {
                prev = Some(Group {
                    c,
                    start: prev_group.start,
                    end: i,
                });
            } else {
                groups.push(prev_group);
                prev = Some(Group {
                    c,
                    start: i,
                    end: i,
                });
            }
        } else {
            prev = Some(Group {
                c,
                start: i,
                end: i,
            });
        }
    }
    if let Some(prev_group) = prev {
        groups.push(prev_group);
    }
    groups
        .iter()
        .filter_map(|g| {
            let start = g.start;
            let end = g.end;
            if end - start >= 2 {
                Some(vec![start as i32, end as i32])
            } else {
                None
            }
        })
        .collect()
}

struct Group {
    c: char,
    start: usize,
    end: usize,
}

// array
#[test]
fn test1_830() {
    assert_eq!(
        large_group_positions(String::from("abbxxxxzzy")),
        vec![vec![3, 6]]
    );
    assert_eq!(
        large_group_positions(String::from("abc")),
        vec![] as Vec<Vec<i32>>
    );
    assert_eq!(
        large_group_positions(String::from("abcdddeeeeaabbbcd")),
        vec![vec![3, 5], vec![6, 9], vec![12, 14]]
    );
}
