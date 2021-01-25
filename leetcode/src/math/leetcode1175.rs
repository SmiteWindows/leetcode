// https://leetcode-cn.com/problems/prime-arrangements/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn num_prime_arrangements(n: i32) -> i32 {
    let primes = number_of_primes(n as usize);
    let mut product = 1_i64;
    for i in 1..=primes {
        product *= i as i64;
        product %= 1_000_000_007;
    }
    for i in 1..=(n - primes) {
        product *= i as i64;
        product %= 1_000_000_007;
    }
    product as i32
}

fn number_of_primes(n: usize) -> i32 {
    let mut a = vec![true; n + 1];
    a[0] = false;
    a[1] = false;
    let mut i = 2_usize;
    while i * i <= n {
        if a[i] {
            let mut j = 2_usize;
            while i * j <= n {
                a[i * j] = false;
                j += 1;
            }
        }
        i += 1;
    }
    let mut res = 0;
    for &k in a.iter().take(n + 1) {
        if k {
            res += 1;
        }
    }
    res
}
// math
#[test]
fn test1_1175() {
    assert_eq!(num_prime_arrangements(5), 12);
    assert_eq!(num_prime_arrangements(100), 682289015);
}
