mod helper;

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n < 1 {
            return Vec::with_capacity(0);
        }

        use std::collections::HashSet;
        fn generate(n: usize) -> HashSet<Vec<char>> {
            if n == 1 {
                let mut parenthesis = HashSet::with_capacity(1);
                parenthesis.insert(vec!['(', ')']);
                return parenthesis;
            }

            let mut set = HashSet::new();
            let v = generate(n - 1);
            let mut tmp = Vec::<char>::with_capacity(n * 2);
            for parenthesis in v {
                for i in 0..parenthesis.len() {
                    tmp.push('(');
                    tmp.extend_from_slice(&parenthesis[0..i]);
                    tmp.push(')');
                    tmp.extend_from_slice(&parenthesis[i..]);

                    set.insert(tmp.clone());
                    tmp.clear();
                }
            }
            set
        }

        let parenthesis = generate(n as usize);
        use std::iter::FromIterator;
        parenthesis
            .into_iter()
            .map(|x| String::from_iter(x.iter()))
            .collect()
    }
}

fn main() {
    let solution = Solution::generate_parenthesis(3);
    println!("solution = {:?}", solution);
    assert_eq!(5, solution.len());
}
