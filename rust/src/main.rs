mod helper;

struct Solution;

impl Solution {
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        assert!(n > 0 && m > 0);
        if n == 1 {
            return n;
        }
        let n = n as usize;
        let m = m as usize;
        let mut arr = vec![0; n];
        for i in 0..n {
            arr[i] = i;
        }
        let mut i = 0;
        loop {
            i = (i + m - 1) % arr.len();
            arr.remove(i);
            if arr.len() == 1 {
                break;
            }
        }
        arr[0] as i32
    }
}

fn main() {
    assert_eq!(3, Solution::last_remaining(5, 3));
    assert_eq!(2, Solution::last_remaining(10, 17));
}
