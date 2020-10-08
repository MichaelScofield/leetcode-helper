mod helper;

struct Solution;

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut diff = vec![0; n as usize];
        for booking in bookings.iter() {
            diff[booking[0] as usize - 1] += booking[2];
            if booking[1] < n {
                diff[booking[1] as usize] -= booking[2];
            }
        }
        for i in 1..diff.len() {
            diff[i] += diff[i - 1];
        }
        diff
    }
}

fn main() {
    assert_eq!(vec![10, 55, 45, 25, 25], Solution::corp_flight_bookings(vecvec![[1,2,10],[2,3,20],[2,5,25]], 5));
}
