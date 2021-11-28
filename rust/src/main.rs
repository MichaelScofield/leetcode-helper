mod helper;

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        assert!(nums.len() >= 1);

        let mut k = 1;
        while k < nums.len() {
            if nums[k] < nums[k - 1] {
                break;
            }
            k += 1;
        }

        let get = |i: usize| -> i32 { nums[(i + k) % nums.len()] };
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i <= j {
            let mid = i + (j - i) / 2;
            if get(mid) == target {
                return true;
            } else if get(mid) < target {
                i = mid + 1;
            } else {
                j = if mid == 0 { break } else { mid - 1 };
            }
        }
        false
    }
}

fn main() {
    assert!(Solution::search(vec![1, 2, 3, 4], 4));
    assert!(Solution::search(vec![1, 1, 1], 1));
    assert!(!Solution::search(vec![1, 1, 1], 2));
    assert!(Solution::search(vec![1, 2, 3, 4], 1));
    assert!(!Solution::search(vec![1, 2, 3, 4], 5));
    assert!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
}
