public class Solution {

    public boolean isMatch(String s, String p) {
        if (s == null && p == null) {
            return true;
        }
        if (s == null || p == null) {
            return false;
        }
        int m = s.length();
        int n = p.length();
        if (m == 0 && n == 0) {
            return true;
        }
        boolean[][] dp = new boolean[m][n];
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                boolean match;
                if (i == 0 && j == 0) {
                    match = true;
                } else if (i == 0 || j == 0) {
                    match = false;
                } else {
                    char cs = s.charAt(i);
                    char cp = p.charAt(j);
                    match = dp[i - 1][j - 1] & (cs == cp || cp == '.');
                    if (!match && cp == '*') {
                        char last = p.charAt(j - 1);
                        match = dp[i - 1][j - 1] & (cs == last || last == '.');
                        if (!match && j >= 2) {
                            match = dp[i][j - 2];
                        }
                    }
                }
                dp[i][j] = match;
            }
        }
        return dp[m - 1][n - 1];
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        System.out.println(!solution.isMatch("aa", "a"));
        System.out.println(solution.isMatch("aa", "a*"));
        System.out.println(solution.isMatch("ab", ".*"));
        System.out.println(solution.isMatch("aab", "c*a*b"));
        System.out.println(!solution.isMatch("mississippi", "mis*is*p*."));
        System.out.println(solution.isMatch("aaa", "a*a"));
        System.out.println(solution.isMatch("aaa", "a*aa"));
        System.out.println(solution.isMatch("aaa", "a*aaa"));
        System.out.println(!solution.isMatch("aaa", "a*aaaa"));
    }
}
