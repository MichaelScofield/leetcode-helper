public class Solution {

    int cells = 0;

    public int movingCount(int m, int n, int k) {
        if (k == 0) {
            return 1;
        }
        boolean[][] visited = new boolean[m][n];
        backtrace(m, n, k, visited, 0, 0);
        return cells;
    }

    void backtrace(int m, int n, int k, boolean[][] visited, int i, int j) {
        // going Right:
        if (j < n - 1 && !visited[i][j + 1] && canEnter(i, j + 1, k)) {
            visited[i][j + 1] = true;
            cells += 1;
            backtrace(m, n, k, visited, i, j + 1);
        }

        // going Down:
        if (i < m - 1 && !visited[i + 1][j] && canEnter(i + 1, j, k)) {
            visited[i + 1][j] = true;
            cells += 1;
            backtrace(m, n, k, visited, i + 1, j);
        }

        // going Left:
        if (j > 0 && !visited[i][j - 1] && canEnter(i, j - 1, k)) {
            visited[i][j - 1] = true;
            cells += 1;
            backtrace(m, n, k, visited, i, j - 1);
        }

        // going Up:
        if (i > 0 && !visited[i - 1][j] && canEnter(i - 1, j, k)) {
            visited[i - 1][j] = true;
            cells += 1;
            backtrace(m, n, k, visited, i - 1, j);
        }
    }

    boolean canEnter(int i, int j, int k) {
        int x = 0;
        while (i > 0) {
            x += i % 10;
            i /= 10;
        }
        int y = 0;
        while (j > 0) {
            y += j % 10;
            j /= 10;
        }
        return x + y <= k;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        System.out.println(solution.movingCount(2, 3, 1));
        System.out.println(solution.movingCount(3, 1, 0));
    }
}
