mod helper;

struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut p = 0;
        while p <= j {
            if nums[p] == 0 {
                nums.swap(i, p);
                i += 1;
                p += 1;
            } else if nums[p] == 2 {
                nums.swap(j, p);
                if j == 0 {
                    return;
                } else {
                    j -= 1;
                }
            } else {
                p += 1;
            }
        }
    }
}

fn main() {
    let mut tests = vec![
        vec![2, 0, 2, 1, 1, 0],
        vec![1, 1, 1, 1, 1, 1],
        vec![0, 0, 0, 0],
        vec![2, 2, 2, 2],
        vec![2, 0, 1],
        vec![1, 2, 0],
        vec![0, 1, 2],
        vec![0, 2, 1],
        vec![1, 0],
        vec![2, 0],
        vec![0, 1],
        vec![0, 2],
        vec![2],
        vec![1],
        vec![0]];
    for test in tests.iter_mut() {
        Solution::sort_colors(test);
        println!("{:?}", test);
    }
}
