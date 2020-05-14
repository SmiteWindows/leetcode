// https://leetcode.com/problems/count-primes/
// Runtime: 4 ms
// Memory Usage: 3.3 MB
pub fn count_primes(n: i32) -> i32 {
    if n <= 2 {
        return 0;
    }
    let n = n as usize;
    let mut a = vec![true; n];
    a[0] = false;
    a[1] = false;
    let mut i = 2;
    while i * i < n {
        if a[i] {
            let mut j = 2;
            while j * i < n {
                a[i * j] = false;
                j += 1;
            }
        }
        i += 1;
    }
    a.iter().fold(0, |sum, &b| if b { sum + 1 } else { sum })
}
// math hash_table
#[test]
fn test1_204() {
    assert_eq!(count_primes(10), 4);
}
