mod helper;

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n < 1 {
            return Vec::with_capacity(0);
        }

        fn is_valid(parenthesis: &Vec<char>) -> bool {
            let len = parenthesis.len();
            let mut tmp = Vec::with_capacity(len);
            for &x in parenthesis {
                if x == ')' {
                    if let Some(&top) = tmp.last() {
                        if top == '(' {
                            tmp.pop();
                            continue;
                        }
                    }
                }
                tmp.push(x);
            }
            tmp.is_empty()
        }

        fn backtrace(solution: &mut Vec<String>, len: usize, parenthesis: &mut Vec<char>) {
            if parenthesis.len() == len {
                if is_valid(parenthesis) {
                    solution.push(parenthesis.clone().into_iter().collect());
                }
                return;
            }

            for &c in ['(', ')'].iter() {
                parenthesis.push(c);
                backtrace(solution, len, parenthesis);
                parenthesis.pop();
            }
        }

        let mut solution = vec![];
        let len = n as usize * 2;
        let mut parenthesis = vec![];
        backtrace(&mut solution, len, &mut parenthesis);
        solution
    }
}

fn main() {
    let solution = Solution::generate_parenthesis(3);
    eprintln!("solution = {:?}", solution);
    assert_eq!(5, solution.len());
}
