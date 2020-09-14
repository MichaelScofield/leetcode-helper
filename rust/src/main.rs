mod helper;

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len = nums.len();
        assert!(len >= 1 && k >= 1 && k <= len as i32);
        let k = k as usize;
        use std::collections::VecDeque;
        let monotonic_queue = &mut VecDeque::new();
        fn enqueue(monotonic_queue: &mut VecDeque<i32>, v: i32) {
            while let Some(&back) = monotonic_queue.back() {
                if back < v {
                    monotonic_queue.pop_back();
                } else {
                    break;
                }
            }
            monotonic_queue.push_back(v);
        }
        fn dequeue(monotonic_queue: &mut VecDeque<i32>, expect: i32) {
            if let Some(&v) = monotonic_queue.front() {
                if v == expect {
                    monotonic_queue.pop_front();
                }
            }
        }
        fn max(monotonic_queue: &VecDeque<i32>) -> i32 {
            *monotonic_queue.front().unwrap()
        }
        let mut sliding_window_max = Vec::with_capacity(len - k + 1);
        for i in 0..len {
            if i < k {
                enqueue(monotonic_queue, nums[i]);
            } else {
                sliding_window_max.push(max(monotonic_queue));
                enqueue(monotonic_queue, nums[i]);
                dequeue(monotonic_queue, nums[i - k]);
            }
        }
        sliding_window_max.push(max(monotonic_queue));
        sliding_window_max
    }
}

fn main() {
    assert_eq!(vec![3, 3, 5, 5, 6, 7],
               Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3));
    assert_eq!(vec![7],
               Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 8));
}
