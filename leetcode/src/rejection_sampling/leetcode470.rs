// https://leetcode-cn.com/problems/implement-rand10-using-rand7/
// Runtime: 8 ms
// Memory Usage: 3 MB
use rand::{distributions::Uniform, prelude::*};
pub fn rand10() -> i32 {
    let distribution: Uniform<i32> = Uniform::new(0, 10);
    let mut rng = thread_rng();
    rng.sample(distribution) + 1
}
/**
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */
fn rand7() -> i32 {
    thread_rng().gen_range(1, 7)
}
// random rejection_sampling
#[test]
fn test1_470() {
    for _ in 0..100 {
        let res1 = rand10();
        match res1 {
            1 => print!("1"),
            2 => print!("2"),
            3 => print!("3"),
            4 => print!("4"),
            5 => print!("5"),
            6 => print!("6"),
            7 => print!("7"),
            8 => print!("8"),
            9 => print!("9"),
            10 => println!("10"),
            _ => panic!(),
        }
    }
}
