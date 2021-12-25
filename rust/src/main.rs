mod helper;

struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        assert!(m >= 1);
        let n = board[0].len();
        assert!(n >= 1);

        let mut taints = vec![];
        for i in 0..m {
            if board[i][0] == 'O' {
                board[i][0] = ' ';
                taints.push((i, 0));
            }
            if board[i][n - 1] == 'O' {
                board[i][n - 1] = ' ';
                taints.push((i, n - 1));
            }
        }
        for j in 0..n {
            if board[0][j] == 'O' {
                board[0][j] = ' ';
                taints.push((0, j));
            }
            if board[m - 1][j] == 'O' {
                board[m - 1][j] = ' ';
                taints.push((m - 1, j));
            }
        }

        while let Some((i, j)) = taints.pop() {
            if i > 0 {
                if board[i - 1][j] == 'O' {
                    board[i - 1][j] = ' ';
                    taints.push((i - 1, j));
                }
            }
            if i < m - 1 {
                if board[i + 1][j] == 'O' {
                    board[i + 1][j] = ' ';
                    taints.push((i + 1, j));
                }
            }
            if j > 0 {
                if board[i][j - 1] == 'O' {
                    board[i][j - 1] = ' ';
                    taints.push((i, j - 1));
                }
            }
            if j < n - 1 {
                if board[i][j + 1] == 'O' {
                    board[i][j + 1] = ' ';
                    taints.push((i, j + 1));
                }
            }
        }

        for i in 1..m - 1 {
            for j in 1..n - 1 {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == ' ' {
                    board[i][j] = 'O';
                }
            }
        }
    }
}

fn main() {
    use helper::util::cast;
    use helper::util::print_matrix;

    let mut board = cast(vecvec![
        [
            "X", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X",
            "X", "X", "X"
        ],
        [
            "X", "X", "X", "X", "X", "X", "X", "X", "X", "O", "O", "O", "X", "X", "X", "X", "X",
            "X", "X", "X"
        ],
        [
            "X", "X", "X", "X", "X", "O", "O", "O", "X", "O", "X", "O", "X", "X", "X", "X", "X",
            "X", "X", "X"
        ],
        [
            "X", "X", "X", "X", "X", "O", "X", "O", "X", "O", "X", "O", "O", "O", "X", "X", "X",
            "X", "X", "X"
        ],
        [
            "X", "X", "X", "X", "X", "O", "X", "O", "O", "O", "X", "X", "X", "X", "X", "X", "X",
            "X", "X", "X"
        ],
        [
            "X", "X", "X", "X", "X", "O", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X",
            "X", "X", "X"
        ]
    ]);
    Solution::solve(&mut board);
    print_matrix(&board);
    assert_eq!(
        cast(vecvec![
            [
                "X", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X",
                "X", "X", "X", "X"
            ],
            [
                "X", "X", "X", "X", "X", "X", "X", "X", "X", "O", "O", "O", "X", "X", "X", "X",
                "X", "X", "X", "X"
            ],
            [
                "X", "X", "X", "X", "X", "O", "O", "O", "X", "O", "X", "O", "X", "X", "X", "X",
                "X", "X", "X", "X"
            ],
            [
                "X", "X", "X", "X", "X", "O", "X", "O", "X", "O", "X", "O", "O", "O", "X", "X",
                "X", "X", "X", "X"
            ],
            [
                "X", "X", "X", "X", "X", "O", "X", "O", "O", "O", "X", "X", "X", "X", "X", "X",
                "X", "X", "X", "X"
            ],
            [
                "X", "X", "X", "X", "X", "O", "X", "X", "X", "X", "X", "X", "X", "X", "X", "X",
                "X", "X", "X", "X"
            ]
        ]),
        board
    );

    let mut board = cast(vecvec![
        ["X", "O", "X", "O", "X", "O", "O", "O", "X", "O"],
        ["X", "O", "O", "X", "X", "X", "O", "O", "O", "X"],
        ["O", "O", "O", "O", "O", "O", "O", "O", "X", "X"],
        ["O", "O", "O", "O", "O", "O", "X", "O", "O", "X"],
        ["O", "O", "X", "X", "O", "X", "X", "O", "O", "O"],
        ["X", "O", "O", "X", "X", "X", "O", "X", "X", "O"],
        ["X", "O", "X", "O", "O", "X", "X", "O", "X", "O"],
        ["X", "X", "O", "X", "X", "O", "X", "O", "O", "X"],
        ["O", "O", "O", "O", "X", "O", "X", "O", "X", "O"],
        ["X", "X", "O", "X", "X", "X", "X", "O", "O", "O"]
    ]);
    Solution::solve(&mut board);
    assert_eq!(
        vecvec![
            ['X', 'O', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'O'],
            ['X', 'O', 'O', 'X', 'X', 'X', 'O', 'O', 'O', 'X'],
            ['O', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'X', 'X'],
            ['O', 'O', 'O', 'O', 'O', 'O', 'X', 'O', 'O', 'X'],
            ['O', 'O', 'X', 'X', 'O', 'X', 'X', 'O', 'O', 'O'],
            ['X', 'O', 'O', 'X', 'X', 'X', 'X', 'X', 'X', 'O'],
            ['X', 'O', 'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O'],
            ['X', 'X', 'O', 'X', 'X', 'X', 'X', 'O', 'O', 'X'],
            ['O', 'O', 'O', 'O', 'X', 'X', 'X', 'O', 'X', 'O'],
            ['X', 'X', 'O', 'X', 'X', 'X', 'X', 'O', 'O', 'O']
        ],
        board
    );

    let mut board = cast(vecvec![
        ["O", "X", "X", "O", "X"],
        ["X", "O", "O", "X", "O"],
        ["X", "O", "X", "O", "X"],
        ["O", "X", "O", "O", "O"],
        ["X", "X", "O", "X", "O"]
    ]);
    Solution::solve(&mut board);
    assert_eq!(
        vecvec![
            ['O', 'X', 'X', 'O', 'X'],
            ['X', 'X', 'X', 'X', 'O'],
            ['X', 'X', 'X', 'O', 'X'],
            ['O', 'X', 'O', 'O', 'O'],
            ['X', 'X', 'O', 'X', 'O']
        ],
        board
    );

    let mut board = cast(vecvec![
        ["X", "X", "X", "X"],
        ["X", "O", "O", "X"],
        ["X", "X", "O", "X"],
        ["X", "O", "X", "X"]
    ]);
    Solution::solve(&mut board);
    assert_eq!(
        vecvec![
            ['X', 'X', 'X', 'X'],
            ['X', 'X', 'X', 'X'],
            ['X', 'X', 'X', 'X'],
            ['X', 'O', 'X', 'X']
        ],
        board
    );

    let mut board = cast(vecvec![["X"]]);
    Solution::solve(&mut board);
    assert_eq!(vecvec![['X']], board);

    let mut board = cast(vecvec![["O"]]);
    Solution::solve(&mut board);
    assert_eq!(vecvec![['O']], board);
}
