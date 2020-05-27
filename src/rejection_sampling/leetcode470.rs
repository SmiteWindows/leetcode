// https://leetcode.com/problems/implement-rand10-using-rand7/

use rand::prelude::*;
pub fn rand10() -> i32 {
    let mut num;
    loop {
        num = rand7() + (rand7() - 1) * 7;
        if num <= 40 {
            return 1 + (num - 1) % 10;
        }

        num = rand7() + (num - 40 - 1) * 7;
        if num <= 60 {
            return 1 + (num - 1) % 10;
        }

        num = rand7() + (num - 60 - 1) * 7;
        if num <= 20 {
            return 1 + (num - 1) % 10;
        }
    }
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
