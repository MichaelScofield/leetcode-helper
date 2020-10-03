mod helper;

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();
        if len < 1 {
            return 0;
        }
        let mut rain = 0;
        let mut i = 0;
        let mut j = len - 1;
        let mut last_sink_height = 0;
        while i < j {
            while i < j {
                if height[i] > last_sink_height {
                    break;
                }
                i += 1;
            }
            while i < j {
                if height[j] > last_sink_height {
                    break;
                }
                j -= 1;
            }
            if i < j {
                let sink_height = std::cmp::min(height[i], height[j]);
                for k in i + 1..j {
                    if height[k] <= last_sink_height {
                        rain += sink_height - last_sink_height;
                    } else if height[k] < sink_height {
                        rain += sink_height - height[k];
                    }
                }
                last_sink_height = sink_height;
            }
        }
        rain
    }
}

fn main() {
    assert_eq!(6, Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    assert_eq!(1, Solution::trap(vec![100, 0, 1]));
    assert_eq!(0, Solution::trap(vec![100, 1]));
    assert_eq!(2, Solution::trap(vec![2, 0, 2]));
}
