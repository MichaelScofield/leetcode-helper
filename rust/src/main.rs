mod helper;

struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if x == 0.0 {
            return 0.0;
        }
        if n == 1 {
            return x;
        }
        let m = n.abs() as usize;
        let mut dp = vec![0.0; m + 1];
        fn pow(x: f64, n: usize, dp: &mut Vec<f64>) -> f64 {
            if n == 1 {
                return x;
            }
            if dp[n] != 0.0 {
                return dp[n];
            }
            let m = n / 2;
            dp[m] = pow(x, m, dp);
            if n % 2 == 1 {
                dp[n - m] = pow(x, n - m, dp);
            }
            dp[m] * dp[n - m]
        }
        let result = pow(x, m, &mut dp);
        if n > 0 { result } else { 1.0 / result }
    }
}

fn main() {
    println!("{}", Solution::my_pow(0.00001, 2147483647));
    println!("{}", Solution::my_pow(2.0, 10));
    println!("{}", Solution::my_pow(2.1, 3));
    println!("{}", Solution::my_pow(2.0, -2));
    println!("{}", Solution::my_pow(0.0, 9));
    println!("{}", Solution::my_pow(1.0, 8));
}
