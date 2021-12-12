mod helper;

struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        assert!(n >= 1);
        Solution::solve_n_queens(n).len() as i32
    }

    fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn place_queue(board: &mut Vec<Vec<u8>>, i: usize, start: usize) -> i32 {
            let mut j = -1;
            for (y, x) in board[i].iter_mut().skip(start).enumerate() {
                if *x == b'0' {
                    j = (y + start) as i32;
                    break;
                }
            }
            let n = board.len();
            if j < 0 {
                return -1;
            }
            let j = j as usize;
            board[i].iter_mut().for_each(|x| *x = b'.');
            board.iter_mut().for_each(|line| line[j] = b'.');
            let (mut x, mut y) = (i, j);
            loop {
                board[x][y] = b'.';
                if x == n - 1 || y == n - 1 {
                    break;
                }
                x += 1;
                y += 1;
            }
            let (mut x, mut y) = (i, j);
            loop {
                board[x][y] = b'.';
                if x == 0 || y == 0 {
                    break;
                }
                x -= 1;
                y -= 1;
            }
            let (mut x, mut y) = (i, j);
            loop {
                board[x][y] = b'.';
                if x == n - 1 || y == 0 {
                    break;
                }
                x += 1;
                y -= 1;
            }
            let (mut x, mut y) = (i, j);
            loop {
                board[x][y] = b'.';
                if x == 0 || y == n - 1 {
                    break;
                }
                x -= 1;
                y += 1;
            }
            board[i][j] = b'Q';
            j as i32
        }

        fn solve_n_queens(board: &mut Vec<Vec<u8>>, i: usize, res: &mut Vec<Vec<Vec<u8>>>) {
            if i == board.len() {
                res.push(board.to_owned());
                return;
            }
            let mut j = 0;
            loop {
                if j >= board.len() {
                    break;
                }
                let mut board = board.clone();
                let place = place_queue(&mut board, i, j);
                if place >= 0 {
                    solve_n_queens(&mut board, i + 1, res);
                    j = place as usize + 1;
                } else {
                    break;
                }
            }
        }

        let n = n as usize;
        let mut board = vec![vec![b'0'; n]; n];
        let mut res = vec![];
        solve_n_queens(&mut board, 0, &mut res);
        res.into_iter()
            .map(|board| {
                board
                    .into_iter()
                    .map(|line| String::from_utf8(line).unwrap())
                    .collect()
            })
            .collect()
    }
}

fn main() {
    assert_eq!(2, Solution::total_n_queens(4));
    assert_eq!(1, Solution::total_n_queens(1));
}
