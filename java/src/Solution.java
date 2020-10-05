import java.util.*;

public class Solution {

    static class Board {

        final int[][] board;
        final int zero_i;
        final int zero_j;
        final int move;

        Board(int[][] board, int zero_i, int zero_j, int move) {
            this.board = board;
            this.zero_i = zero_i;
            this.zero_j = zero_j;
            this.move = move;
        }

        Board move(int[] movement) {
            int i = zero_i + movement[0];
            int j = zero_j + movement[1];
            if (i < 0 || i >= 2 || j < 0 || j >= 3) {
                return null;
            }

            int[][] board = new int[2][];
            board[0] = this.board[0].clone();
            board[1] = this.board[1].clone();

            int tmp = board[i][j];
            board[i][j] = board[zero_i][zero_j];
            board[zero_i][zero_j] = tmp;
            return new Board(board, i, j, move + 1);
        }

        boolean isSolved() {
            return board[0][0] == 1 && board[0][1] == 2 && board[0][2] == 3 &&
                    board[1][0] == 4 && board[1][1] == 5 && board[1][2] == 0;
        }

        @Override
        public boolean equals(Object o) {
            assert o instanceof Board;
            return Arrays.equals(board[0], ((Board) o).board[0]) &&
                    Arrays.equals(board[1], ((Board) o).board[1]);
        }

        @Override
        public int hashCode() {
            return Arrays.hashCode(board[0]) * 31 + Arrays.hashCode(board[1]);
        }
    }

    public int slidingPuzzle(int[][] board) {
        assert board.length == 2 && board[0].length == 3 && board[1].length == 3;

        Set<Board> visited = new HashSet<>();
        Queue<Board> boards = new LinkedList<>();

        int zero_i = 0;
        int zero_j = 0;
        for (int i = 0; i < board.length; i++) {
            int[] row = board[i];
            for (int j = 0; j < row.length; j++) {
                if (row[j] == 0) {
                    zero_i = i;
                    zero_j = j;
                    break;
                }
            }
        }
        boards.add(new Board(board, zero_i, zero_j, 0));

        int[][] movements = new int[][]{new int[]{-1, 0}, new int[]{1, 0}, new int[]{0, -1}, new int[]{0, 1}};
        int smallestMove = Integer.MAX_VALUE;
        while (!boards.isEmpty()) {
            Board b = boards.poll();
            if (b.isSolved()) {
                smallestMove = Math.min(smallestMove, b.move);
                continue;
            }
            visited.add(b);

            for (int[] movement : movements) {
                Board newBoard = b.move(movement);
                if (newBoard != null && !visited.contains(newBoard)) {
                    boards.add(newBoard);
                }
            }
        }
        return smallestMove == Integer.MAX_VALUE ? -1 : smallestMove;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        System.out.println(0 == solution.slidingPuzzle(new int[][]{new int[]{1, 2, 3}, new int[]{4, 5, 0}}));
        System.out.println(1 == solution.slidingPuzzle(new int[][]{new int[]{1, 2, 3}, new int[]{4, 0, 5}}));
        System.out.println(-1 == solution.slidingPuzzle(new int[][]{new int[]{1, 2, 3}, new int[]{5, 4, 0}}));
        System.out.println(5 == solution.slidingPuzzle(new int[][]{new int[]{4, 1, 2}, new int[]{5, 0, 3}}));
        System.out.println(14 == solution.slidingPuzzle(new int[][]{new int[]{3, 2, 4}, new int[]{1, 5, 0}}));
    }
}
