// https://leetcode.com/problems/generate-random-point-in-a-circle/
// Runtime: 20 ms
// Memory Usage: 9 MB
use rand::prelude::*;
struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
    rng: ThreadRng,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        let rng = rand::thread_rng();
        Self {
            radius,
            x_center,
            y_center,
            rng,
        }
    }

    fn rand_point(&mut self) -> Vec<f64> {
        let mut x = self.rng.gen_range(-self.radius, self.radius);
        let mut y = self.rng.gen_range(-self.radius, self.radius);
        while x * x + y * y > self.radius * self.radius {
            x = self.rng.gen_range(-self.radius, self.radius);
            y = self.rng.gen_range(-self.radius, self.radius);
        }
        vec![x + self.x_center, y + self.y_center]
    }
}
/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */
// random math rejection_sampling
#[test]
fn test1_478() {
    let mut obj = Solution::new(1.0, 0.0, 0.0);
    let ret_1 = obj.rand_point();
    let ret_2 = obj.rand_point();
    let ret_3 = obj.rand_point();
    println!("{:?}",ret_1);
    println!("{:?}",ret_2);
    println!("{:?}",ret_3);
    let mut obj = Solution::new(10.0, 5.0, -7.5);
    let ret_1 = obj.rand_point();
    let ret_2 = obj.rand_point();
    let ret_3 = obj.rand_point();
    println!("{:?}",ret_1);
    println!("{:?}",ret_2);
    println!("{:?}",ret_3);
}
