public class Solution {

    public boolean isMatch(String s, String p) {
        int m = s == null ? 0 : s.length();
        int n = p == null ? 0 : p.length();

        boolean[][] dp = new boolean[m + 1][n + 1];
        dp[0][0] = true;

        for (int j = 1; j <= n; j++) {
            dp[0][j] = p.charAt(j - 1) == '*' && dp[0][j - 1];
        }

        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                char cs = s.charAt(i - 1);
                char cp = p.charAt(j - 1);
                if (cp == cs || cp == '?') {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if (cp == '*') {
                    dp[i][j] = dp[i - 1][j] || dp[i][j - 1];
                } else {
                    dp[i][j] = false;
                }
            }
        }
        return dp[m][n];
    }

    public static void main(String[] args) {
        Solution solution = new Solution();

        System.out.println(!solution.isMatch("aa", "a"));
        System.out.println(solution.isMatch("aa", "*"));
        System.out.println(!solution.isMatch("cb", "?a"));
        System.out.println(solution.isMatch("adceb", "*a*b"));
        System.out.println(!solution.isMatch("acdcb", "a*c?b"));
    }
}
