public class Solution {

    public boolean searchMatrix(int[][] matrix, int target) {
        if (matrix == null || matrix.length == 0) {
            return false;
        }
        int m = matrix[0].length;
        int n = matrix.length;
        for (int i = m - 1, j = 0; i >= 0 && j < n; ) {
            if (matrix[j][i] == target) {
                return true;
            }
            if (matrix[j][i] < target) {
                j += 1;
            } else {
                i -= 1;
            }
        }
        return false;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        int[][] nums = new int[][]{
                new int[]{1, 4, 7, 11, 15},
                new int[]{2, 5, 8, 12, 19},
                new int[]{3, 6, 9, 16, 22},
                new int[]{10, 13, 14, 17, 24},
                new int[]{18, 21, 23, 26, 30}
        };
        System.out.println(solution.searchMatrix(nums, 5));
        System.out.println(solution.searchMatrix(nums, 20));
        System.out.println(solution.searchMatrix(nums, 1));
        System.out.println(solution.searchMatrix(nums, 30));
        System.out.println(solution.searchMatrix(nums, 31));
    }
}
