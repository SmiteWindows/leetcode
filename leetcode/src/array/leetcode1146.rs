// https://leetcode-cn.com/problems/snapshot-array/
// Runtime: 56 ms
// Memory Usage: 24 MB
struct SnapshotArray {
    length: usize,
    data: Vec<Vec<(usize, i32)>>,
    snap_id: usize,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        let length = length as usize;
        let data = vec![vec![(0, 0)]; length];
        let snap_id = 0;
        Self {
            length,
            data,
            snap_id,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        let i = index as usize;
        if let Some(last) = self.data[i].pop() {
            if last.0 != self.snap_id {
                self.data[i].push(last);
            }
        }
        self.data[i].push((self.snap_id, val));
    }

    fn snap(&mut self) -> i32 {
        let curr_id: i32 = self.snap_id as i32;
        self.snap_id += 1;
        curr_id
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let i = index as usize;
        let snap_id = snap_id as usize;
        match self.data[i].binary_search_by_key(&snap_id, |p| p.0) {
            Ok(j) => self.data[i][j].1,
            Err(j) => self.data[i][j - 1].1,
        }
    }
}
/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */
// array
#[test]
fn test1_1146() {
    let mut obj = SnapshotArray::new(3);
    obj.set(0, 5);
    assert_eq!(obj.snap(), 0);
    obj.set(0, 6);
    assert_eq!(obj.get(0, 0), 5);
}
