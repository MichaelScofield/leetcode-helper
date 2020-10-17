mod helper;

struct Solution;

impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let arr = &mut { arr };
        arr.sort();
        let mut sum = 0f64;
        let l = arr.len() * 5 / 100;
        let r = arr.len() - l;
        for i in l..r {
            sum += arr[i] as f64;
        }
        sum / (r - l) as f64
    }
}

fn main() {
    println!("{:?}", Solution::trim_mean(vec![6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5, 10, 8, 6, 6, 1, 0, 6, 10, 8, 2, 3, 4]));
}
