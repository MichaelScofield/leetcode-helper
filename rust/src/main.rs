mod helper;

struct Solution;

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let m = nums.len();
        assert!(m >= 1);
        let (size, n) = nums.iter()
            .map(|row| (row.len(), row.len()))
            .fold((0, 0), |acc, x|
                (acc.0 + x.0, std::cmp::max(acc.1, x.1)));
        let mut result = Vec::with_capacity(size);
        for i in 0..m {
            let mut x = i;
            for j in 0..n {
                if j < nums[x].len() {
                    result.push(nums[x][j]);
                }
                if x == 0 {
                    break;
                }
                x -= 1;
            }
        }
        for j in 1..n {
            let mut y = j;
            for i in (0..m).rev() {
                if y < nums[i].len() {
                    result.push(nums[i][y]);
                }
                y += 1;
                if y >= n {
                    break;
                }
            }
        }
        result
    }
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    assert_eq!(vec![1, 4, 2, 7, 5, 3, 8, 6, 9], Solution::find_diagonal_order(matrix));
    let matrix = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7],
        vec![8],
        vec![9, 10, 11],
        vec![12, 13, 14, 15, 16]];
    assert_eq!(vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16],
               Solution::find_diagonal_order(matrix));
    let matrix = vec![
        vec![1, 2, 3],
        vec![4],
        vec![5, 6, 7],
        vec![8],
        vec![9, 10, 11]];
    assert_eq!(vec![1, 4, 2, 5, 3, 8, 6, 9, 7, 10, 11], Solution::find_diagonal_order(matrix));
    let matrix = vec![vec![1, 2, 3, 4, 5, 6]];
    assert_eq!(vec![1, 2, 3, 4, 5, 6], Solution::find_diagonal_order(matrix));
}
