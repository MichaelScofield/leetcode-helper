mod helper;

struct Solution;

impl Solution {
    pub fn two_sum(n: i32) -> Vec<f64> {
        assert!(n > 0);
        let n = n as usize;
        fn backtrack(n: usize, (touzi, s): (&mut Vec<usize>, usize), sums: &mut Vec<i32>) {
            if touzi.len() == n {
                sums[s - n] += 1;
                return;
            }
            for dian in 1..=6 {
                touzi.push(dian);
                backtrack(n, (touzi, s + dian), sums);
                touzi.pop();
            }
        }
        let mut sums = vec![0; 5 * n + 1];
        let mut touzi = Vec::with_capacity(n);
        backtrack(n, (&mut touzi, 0), &mut sums);
        let total = 6i32.pow(n as u32) as f64;
        sums.iter().map(|&s| s as f64 / total).collect()
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(1));
    println!("{:?}", Solution::two_sum(2));
    println!("{:?}", Solution::two_sum(11));
}
