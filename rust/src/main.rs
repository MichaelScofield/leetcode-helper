mod helper;

struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let len = piles.len();
        assert!(len > 0);
        assert!(h >= len as i32);

        let piles = &mut { piles };
        piles.sort();

        let mut i = 1;
        let mut j = piles[len - 1];
        while i <= j {
            let k = i + (j - i) / 2;
            let eat_up_hours = piles.iter().map(|&pile| (pile + k - 1) / k).sum::<i32>();
            if eat_up_hours <= h {
                j = k - 1;
            } else {
                i = k + 1;
            }
        }
        j + 1
    }
}

fn main() {
    assert_eq!(4, Solution::min_eating_speed(vec![3, 6, 7, 11], 8));
    assert_eq!(30, Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5));
    assert_eq!(23, Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6));
}
