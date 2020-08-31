public class Solution {

    public int integerBreak(int n) {
        assert n > 0;
        if (n == 1 || n == 2) {
            return 1;
        }
        if (n == 3) {
            return 2;
        }
        int[] dp = new int[n + 1];
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 3;
        for (int i = 4; i <= n; i++) {
            int max = 0;
            for (int j = 0; j <= i; j++) {
                max = Math.max(max, dp[j] * dp[i - j]);
            }
            dp[i] = max;
        }
        return dp[n];
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        System.out.println(solution.integerBreak(2) == 1);
        System.out.println(solution.integerBreak(10) == 36);
    }
}
