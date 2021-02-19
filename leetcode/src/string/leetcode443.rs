// https://leetcode-cn.com/problems/string-compression/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut j = 0;
    let mut prev = None;
    let mut count = 0;
    let n = chars.len();
    for i in 0..n {
        if let Some(c) = prev {
            if c == chars[i] {
                count += 1;
            } else {
                j += write_pair(chars, j, c, count);
                prev = Some(chars[i]);
                count = 1;
            }
        } else {
            count = 1;
            prev = Some(chars[i]);
        }
    }
    if let Some(c) = prev {
        j += write_pair(chars, j, c, count);
    }
    j as i32
}

fn write_pair(chars: &mut Vec<char>, mut index: usize, c: char, mut count: usize) -> usize {
    chars[index] = c;
    index += 1;
    if count == 1 {
        return 1;
    }
    let mut size = 0;
    while count > 0 {
        let d = count as u8 % 10_u8 + b'0';
        chars[index + size] = d as char;
        size += 1;
        count /= 10;
    }
    let mut i = index;
    let mut j = index + size - 1;
    while i < j {
        chars.swap(i, j);
        i += 1;
        j -= 1;
    }
    1 + size
}
// string
#[test]
fn test1_443() {
    let mut chars1 = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    assert_eq!(compress(&mut chars1), 6);
    let mut chars2 = vec!['a'];
    assert_eq!(compress(&mut chars2), 1);
    let mut chars3 = vec![
        'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
    ];
    assert_eq!(compress(&mut chars3), 4);
}
