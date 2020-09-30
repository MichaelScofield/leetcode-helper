mod helper;

struct Solution;

impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        assert!(k >= 0);

        fn left_bound(k: i64) -> i64 {
            let mut l = 0;
            let mut h = std::i64::MAX;
            while l <= h {
                let mid = l + (h - l) / 2;
                let zeroes = Solution::trailing_zeroes(mid);
                if zeroes == k {
                    h = mid - 1;
                } else if zeroes < k {
                    l = mid + 1;
                } else {
                    h = mid - 1;
                }
            }
            h + 1
        }

        fn right_bound(k: i64) -> i64 {
            let mut l = 0;
            let mut h = std::i64::MAX;
            while l <= h {
                let mid = l + (h - l) / 2;
                let zeroes = Solution::trailing_zeroes(mid);
                if zeroes == k {
                    l = mid + 1;
                } else if zeroes < k {
                    l = mid + 1;
                } else {
                    h = mid - 1;
                }
            }
            l - 1
        }

        let k = k as i64;
        (right_bound(k) - left_bound(k) + 1) as i32
    }

    fn trailing_zeroes(n: i64) -> i64 {
        let mut zeroes = 0;
        let mut d = 1;
        // expect "d * 5 <= n", shift multiply to the right side to avoid overflow
        while d <= n / 5 {
            d *= 5;
            zeroes += n / d;
        }
        zeroes
    }
}

fn main() {
    assert_eq!(5, Solution::preimage_size_fzf(0));
    assert_eq!(0, Solution::preimage_size_fzf(5));
}
