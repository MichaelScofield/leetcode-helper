public class Solution {

    public boolean isSubStructure(TreeNode A, TreeNode B) {
        if (A == null || B == null) {
            return false;
        }
        if (isSameLargerTree(A, B)) {
            return true;
        }
        return isSubStructure(A.left, B) || isSubStructure(A.right, B);
    }

    boolean isSameLargerTree(TreeNode ra, TreeNode rb) {
        if (rb == null) {
            return true;
        }
        if (ra == null) {
            return false;
        }
        if (ra.val != rb.val) {
            return false;
        }
        return isSameLargerTree(ra.left, rb.left) && isSameLargerTree(ra.right, rb.right);
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        TreeNode A, B;

        A = TreeNode.createTree(new Integer[]{1, 2, 3});
        B = TreeNode.createTree(new Integer[]{3, 1, null});
        System.out.println(!solution.isSubStructure(A, B));

        A = TreeNode.createTree(new Integer[]{3, 4, 5, 1, 2});
        B = TreeNode.createTree(new Integer[]{4, 1, null});
        System.out.println(solution.isSubStructure(A, B));
    }
}
