// https://leetcode-cn.com/problems/asteroid-collision/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    for x in asteroids {
        if x > 0 {
            res.push(x);
        } else {
            collide(&mut res, x);
        }
    }
    res
}

fn collide(stack: &mut Vec<i32>, asteroid: i32) {
    if let Some(&top) = stack.last() {
        if top < 0 {
            stack.push(asteroid);
        } else {
            match top.cmp(&-asteroid) {
                Less => {
                    stack.pop();
                    collide(stack, asteroid);
                }
                Equal => {
                    stack.pop();
                }
                Greater => {}
            }
        }
    } else {
        stack.push(asteroid);
    }
}
// stack
#[test]
fn test1_735() {
    assert_eq!(asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    assert_eq!(asteroid_collision(vec![8, -8]), vec![]);
    assert_eq!(asteroid_collision(vec![10, 2, -5]), vec![10]);
    assert_eq!(asteroid_collision(vec![-2, -1, 1, 2]), vec![-2, -1, 1, 2]);
}
