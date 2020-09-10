public class Solution {

    public TreeNode searchBST(TreeNode root, int val) {
        if (root == null) {
            return null;
        }
        if (root.val == val) {
            return root;
        } else if (root.val < val) {
            return searchBST(root.right, val);
        } else {
            return searchBST(root.left, val);
        }
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        TreeNode root;

        root = TreeNode.createTree(new Integer[]{4, 2, 7, 1, 3});
        root = solution.searchBST(root, 2);
        TreeNode.printTree(root);
        System.out.println();

        root = TreeNode.createTree(new Integer[]{4, 2, 7, 1, 3});
        root = solution.searchBST(root, 0);
        TreeNode.printTree(root);
        System.out.println();

        root = TreeNode.createTree(new Integer[]{});
        root = solution.searchBST(root, 0);
        TreeNode.printTree(root);
        System.out.println();

        root = TreeNode.createTree(new Integer[]{18, 2, 22, null, null, null, 63, null, 84, null, null});
        root = solution.searchBST(root, 63);
        TreeNode.printTree(root);
        System.out.println();

        root = TreeNode.createTree(new Integer[]{5});
        root = solution.searchBST(root, 5);
        TreeNode.printTree(root);
        System.out.println();

        root = TreeNode.createTree(new Integer[]{1, null, 2, null, 3});
        root = solution.searchBST(root, 2);
        TreeNode.printTree(root);
        System.out.println();
    }
}
