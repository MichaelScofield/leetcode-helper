mod helper;

struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        assert!(n >= 1 && k >= 1);
        let mut v = (1..=n).collect::<Vec<i32>>();
        for _ in 1..k {
            Solution::next_permutation(&mut v);
        }
        let mut s = String::with_capacity(v.len());
        for x in v {
            s.push(char::from_digit(x as u32, 10).unwrap())
        }
        s
    }

    fn next_permutation(nums: &mut Vec<i32>) {
        assert!(nums.len() >= 1);

        // find not increasing subsequence's starting index from right
        let mut non_increasing_i = nums.len() - 1;
        while non_increasing_i > 0 {
            if nums[non_increasing_i - 1] < nums[non_increasing_i] {
                break;
            }
            non_increasing_i -= 1;
        }
        // already the largest permutation, reverse to get the smallest
        if non_increasing_i == 0 {
            nums.reverse();
            return;
        }

        // find the num that is just larger
        let mut larger_i = nums.len() - 1;
        while larger_i >= non_increasing_i {
            if nums[larger_i] > nums[non_increasing_i - 1] {
                break;
            }
            larger_i -= 1;
        }

        // swap the larger out
        nums.swap(non_increasing_i - 1, larger_i);

        // reverse to make the new subsequence starting from smallest
        nums[non_increasing_i..].reverse();
    }
}

fn main() {
    assert_eq!("213".to_string(), Solution::get_permutation(3, 3));
    assert_eq!("2314".to_string(), Solution::get_permutation(4, 9));
    assert_eq!("123".to_string(), Solution::get_permutation(3, 1));
}
