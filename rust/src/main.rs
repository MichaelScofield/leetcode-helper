mod helper;

struct Solution;

impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        fn dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> f32 {
            (((x1 - x2).abs().pow(2) + (y1 - y2).abs().pow(2)) as f32).sqrt()
        }

        fn signal(q: i32, d: f32) -> i32 {
            (q as f32 / (1 as f32 + d)).floor() as i32
        }

        let range_points = |(x, y): (i32, i32)| {
            let mut points = vec![];
            for i in std::cmp::max(0, x - radius)..=(x + radius) {
                for j in std::cmp::max(0, y - radius)..=(y + radius) {
                    if dist((i, j), (x, y)) <= radius as f32 {
                        points.push((i, j));
                    }
                }
            }
            points
        };

        use std::collections::HashSet;
        let mut cache = HashSet::new();

        let mut max_signal = ((std::i32::MAX, std::i32::MAX), std::i32::MIN);
        for i in 0..towers.len() {
            let tower = &towers[i];
            let points = range_points((tower[0], tower[1]));
            for point in points {
                if !cache.insert(point) {
                    continue;
                }

                let mut signal_sum = 0;
                for j in 0..towers.len() {
                    let tower = &towers[j];
                    let d = dist(point, (tower[0], tower[1]));
                    if d > radius as f32 {
                        continue;
                    }
                    let q = tower[2];
                    signal_sum += signal(q, d);
                }
                if signal_sum < max_signal.1 {
                    continue;
                }
                if signal_sum > max_signal.1 {
                    max_signal = (point, signal_sum);
                } else {
                    let (min_x, min_y) = max_signal.0;
                    if point.0 < min_x || point.0 == min_x && point.1 < min_y {
                        max_signal = (point, signal_sum);
                    }
                }
            }
        }

        let (x, y) = max_signal.0;
        vec![x, y]
    }
}

fn main() {
    assert_eq!(vec![2, 1], Solution::best_coordinate(vecvec![[1,2,5],[2,1,7],[3,1,9]], 2));
    assert_eq!(vec![23, 11], Solution::best_coordinate(vecvec![[23,11,21]], 9));
    assert_eq!(vec![1, 2], Solution::best_coordinate(vecvec![[1,2,13],[2,1,7],[0,1,9]], 2));
    assert_eq!(vec![0, 1], Solution::best_coordinate(vecvec![[2,1,9],[0,1,9]], 2));
}
