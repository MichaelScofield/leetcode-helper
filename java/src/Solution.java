public class Solution {

    public void flatten(TreeNode root) {
        if (root == null) {
            return;
        }
        TreeNode left = root.left;
        TreeNode right = root.right;
        flatten(left);
        flatten(right);
        if (left != null) {
            TreeNode rightMost = left;
            while (rightMost.right != null) {
                rightMost = rightMost.right;
            }
            rightMost.right = right;
            root.right = left;
            root.left = null;
        }
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        TreeNode root = TreeNode.createTree(new Integer[]{1, 2, 5, 3, 4, null, 6});
        solution.flatten(root);
        TreeNode.printTree(root);
    }
}
