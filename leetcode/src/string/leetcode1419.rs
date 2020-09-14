// https://leetcode-cn.com/problems/minimum-number-of-frogs-croaking/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
    let mut result = 0;
    let mut count = vec![0; 5];
    for c in croak_of_frogs.chars() {
        match c {
            'c' => {
                count[0] += 1;
                if count[0] > result {
                    result = count[0];
                }
            }
            _ => {
                let index = get_index(c);
                count[index] += 1;
                if count[index] > count[index - 1] {
                    return -1;
                }
                if c == 'k' {
                    for v in count.iter_mut() {
                        *v -= 1;
                    }
                }
            }
        }
    }
    if count[0] == 0 {
        result
    } else {
        -1
    }
}

fn get_index(c: char) -> usize {
    match c {
        'r' => 1,
        'o' => 2,
        'a' => 3,
        'k' => 4,
        _ => 0,
    }
}
// string
#[test]
fn test1_1419() {
    assert_eq!(min_number_of_frogs("croakcroak".to_string()), 1);
    assert_eq!(min_number_of_frogs("crcoakroak".to_string()), 2);
    assert_eq!(min_number_of_frogs("croakcrook".to_string()), -1);
    assert_eq!(min_number_of_frogs("croakcroa".to_string()), -1);
}
