// https://leetcode-cn.com/problems/design-underground-system/
// Runtime: 36 ms
// Memory Usage: 16.9 MB
use std::collections::HashMap;
#[derive(Default)]
struct UndergroundSystem {
    time: HashMap<String, HashMap<String, (i32, i32)>>,
    customer: HashMap<i32, (String, i32)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        Self::default()
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.customer.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (start_station, start_t) = self.customer.remove(&id).expect("in");
        let (sum, count) = self
            .time
            .entry(start_station)
            .or_default()
            .entry(station_name)
            .or_default();
        *sum += t - start_t;
        *count += 1;
    }

    fn get_average_time(&mut self, start_station: String, end_station: String) -> f64 {
        let (sum, count) = self
            .time
            .entry(start_station)
            .or_default()
            .entry(end_station)
            .or_default();
        *sum as f64 / *count as f64
    }
}
/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */
// design
#[test]
fn test1_1396() {
    use leetcode_prelude::assert_approx_eq;
    let mut obj = UndergroundSystem::new();
    obj.check_in(45, "Leyton".to_string(), 3);
    obj.check_in(32, "Paradise".to_string(), 8);
    obj.check_in(27, "Leyton".to_string(), 10);
    obj.check_out(45, "Waterloo".to_string(), 15);
    obj.check_out(27, "Waterloo".to_string(), 20);
    obj.check_out(32, "Cambridge".to_string(), 22);
    let t = obj.get_average_time("Paradise".to_string(), "Cambridge".to_string());
    // assert!((t - 14.0).abs() <= f64::EPSILON);
    assert_approx_eq!(t, 14.0);
    let t = obj.get_average_time("Leyton".to_string(), "Waterloo".to_string());
    // assert!((t - 11.0).abs() <= f64::EPSILON);
    assert_approx_eq!(t, 11.0);
    obj.check_in(10, "Leyton".to_string(), 24);
    let t = obj.get_average_time("Leyton".to_string(), "Waterloo".to_string());
    // assert!((t - 11.0).abs() <= f64::EPSILON);
    assert_approx_eq!(t, 11.0);
    obj.check_out(10, "Waterloo".to_string(), 38);
    let t = obj.get_average_time("Leyton".to_string(), "Waterloo".to_string());
    // assert!((t - 12.0).abs() <= f64::EPSILON);
    assert_approx_eq!(t, 12.0);
    let mut obj = UndergroundSystem::new();
    obj.check_in(10, "Leyton".to_string(), 3);
    obj.check_out(10, "Paradise".to_string(), 8);
    let t = obj.get_average_time("Leyton".to_string(), "Paradise".to_string());
    // assert!((t - 5.0).abs() <= f64::EPSILON);
    assert_approx_eq!(t, 5.0);
    obj.check_in(5, "Leyton".to_string(), 10);
    obj.check_out(5, "Paradise".to_string(), 16);
    let t = obj.get_average_time("Leyton".to_string(), "Paradise".to_string());
    // assert!((t - 5.5).abs() <= f64::EPSILON);
    assert_approx_eq!(t, 5.5);
    obj.check_in(2, "Leyton".to_string(), 21);
    obj.check_out(2, "Paradise".to_string(), 30);
    let t = obj.get_average_time("Leyton".to_string(), "Paradise".to_string());
    // assert!((t - 6.66667).abs() <= f64::EPSILON);
    assert_approx_eq!(t, 6.666666666666667);
}
