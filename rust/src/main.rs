mod helper;

struct Solution;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        let len = weights.len();
        assert!(d >= 1);
        assert!(len >= d as usize);

        fn ship_days(weights: &Vec<i32>, capacity: i32) -> i32 {
            let mut days = 0;
            let mut sum = 0;
            let mut i = 0;
            while i < weights.len() {
                sum += weights[i];
                if sum > capacity {
                    days += 1;
                    sum = weights[i];
                }
                i += 1;
            }
            days + 1
        }

        let mut min_capacity = std::i32::MIN;
        let mut max_capacity = 0;
        for &w in weights.iter() {
            min_capacity = std::cmp::max(min_capacity, w);
            max_capacity += w;
        }

        let mut i = min_capacity;
        let mut j = max_capacity;
        while i <= j {
            let capacity = i + (j - i) / 2;
            if ship_days(&weights, capacity) <= d {
                j = capacity - 1;
            } else {
                i = capacity + 1;
            }
        }
        j + 1
    }
}

fn main() {
    assert_eq!(15, Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5));
    assert_eq!(6, Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3));
    assert_eq!(3, Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4));
}
