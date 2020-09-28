mod helper;

struct Solution;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        if envelopes.len() == 0 {
            return 0;
        }
        let envelopes = &mut { envelopes };
        envelopes.sort_by(|a, b|
            a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
        let mut dp = vec![1; envelopes.len()];
        dp[0] = 1;
        for i in 1..envelopes.len() {
            for j in (0..i).rev() {
                if envelopes[i][0] > envelopes[j][0] && envelopes[i][1] > envelopes[j][1] {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1);
                }
            }
        }
        *dp.iter().max().unwrap()
    }
}

fn main() {
    let envelopes = vecvec![[46,89],[50,53],[52,68],[72,45],[77,81]];
    assert_eq!(3, Solution::max_envelopes(envelopes));
    let envelopes = vecvec![[4,5],[4,6],[6,7],[2,3],[1,1],[1,1]];
    assert_eq!(4, Solution::max_envelopes(envelopes));
    assert_eq!(3, Solution::max_envelopes(vecvec![[5,4],[6,4],[6,7],[2,3]]));
    let envelopes =
        vecvec![[2,100],[3,200],[4,300],[5,500],[5,400],[5,250],[6,370],[6,360],[7,380]];
    assert_eq!(5, Solution::max_envelopes(envelopes));
}
