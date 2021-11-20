mod helper;

struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        assert!(candidates.len() >= 1);
        let candidates = &mut { candidates };
        candidates.sort();

        fn combination_sum(
            candidates: &Vec<i32>,
            target: i32,
            result: &mut Vec<i32>,
            results: &mut Vec<Vec<i32>>,
            start: usize,
        ) {
            let mut i = start;
            while i < candidates.len() {
                let candidate = candidates[i];
                if target < candidate {
                    return;
                }
                result.push(candidate);
                if target == candidate {
                    results.push(result.clone());
                } else {
                    combination_sum(&candidates, target - candidate, result, results, i + 1);
                }
                let last = result.pop().unwrap();
                while i < candidates.len() {
                    if candidates[i] != last {
                        break;
                    }
                    i += 1;
                }
            }
        }

        let mut results = vec![];
        let mut result = vec![];
        combination_sum(candidates, target, &mut result, &mut results, 0);
        results
    }
}

fn main() {
    assert_eq!(
        vecvec![[1, 1, 6], [1, 2, 5], [1, 7], [2, 6]],
        Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)
    );
    assert_eq!(
        vecvec![[1, 2, 2], [5]],
        Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5)
    );
    assert_eq!(
        vecvec![[7]],
        Solution::combination_sum2(vec![2, 3, 6, 7], 7)
    );
    assert_eq!(
        vecvec![[3, 5]],
        Solution::combination_sum2(vec![2, 3, 5], 8)
    );
    assert_eq!(
        Vec::<Vec<i32>>::new(),
        Solution::combination_sum2(vec![2], 1)
    );
    assert_eq!(vecvec![[1]], Solution::combination_sum2(vec![1], 1));
    assert_eq!(
        Vec::<Vec<i32>>::new(),
        Solution::combination_sum2(vec![1], 2)
    );
}
