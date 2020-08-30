import java.util.AbstractMap.SimpleImmutableEntry;
import java.util.ArrayList;
import java.util.List;

public class Solution {

    public boolean exist(char[][] board, String word) {
        List<SimpleImmutableEntry<Integer, Integer>> startPositions = new ArrayList<>();
        int m = board.length;
        int n = board[0].length;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (board[i][j] == word.charAt(0)) {
                    startPositions.add(new SimpleImmutableEntry<>(i, j));
                }
            }
        }
        for (SimpleImmutableEntry<Integer, Integer> startPosition : startPositions) {
            boolean[][] visited = new boolean[m][n];
            int i = startPosition.getKey();
            int j = startPosition.getValue();
            visited[i][j] = true;
            if (backtrace(board, word, 1, visited, startPosition)) {
                return true;
            }
        }
        return false;
    }

    boolean backtrace(char[][] board, String word,
                      int k, boolean[][] visited, SimpleImmutableEntry<Integer, Integer> position) {
        if (k == word.length()) {
            return true;
        }
        int m = board.length;
        int n = board[0].length;
        char expected = word.charAt(k);
        int i = position.getKey();
        int j = position.getValue();

        // going Right:
        if (j < n - 1 && board[i][j + 1] == expected && !visited[i][j + 1]) {
            visited[i][j + 1] = true;
            if (backtrace(board, word, k + 1, visited, new SimpleImmutableEntry<>(i, j + 1))) {
                return true;
            }
            visited[i][j + 1] = false;
        }

        // going Down:
        if (i < m - 1 && board[i + 1][j] == expected && !visited[i + 1][j]) {
            visited[i + 1][j] = true;
            if (backtrace(board, word, k + 1, visited, new SimpleImmutableEntry<>(i + 1, j))) {
                return true;
            }
            visited[i + 1][j] = false;
        }

        // going Left:
        if (j > 0 && board[i][j - 1] == expected && !visited[i][j - 1]) {
            visited[i][j - 1] = true;
            if (backtrace(board, word, k + 1, visited, new SimpleImmutableEntry<>(i, j - 1))) {
                return true;
            }
            visited[i][j - 1] = false;
        }

        // going Up:
        if (i > 0 && board[i - 1][j] == expected && !visited[i - 1][j]) {
            visited[i - 1][j] = true;
            if (backtrace(board, word, k + 1, visited, new SimpleImmutableEntry<>(i - 1, j))) {
                return true;
            }
            visited[i - 1][j] = false;
        }
        return false;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        char[][] board = new char[][]{
                new char[]{'A', 'B', 'C', 'E'},
                new char[]{'S', 'F', 'C', 'S'},
                new char[]{'A', 'D', 'E', 'E'}
        };
        System.out.println(solution.exist(board, "ABCCED"));
        System.out.println(solution.exist(board, "SEE"));
        System.out.println(solution.exist(board, "ABCB"));
        System.out.println(solution.exist(board, "A"));
        System.out.println(solution.exist(board, "B"));
        System.out.println(solution.exist(board, "X"));
    }
}
