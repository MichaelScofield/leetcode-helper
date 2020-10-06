public class Solution {

    public TreeNode buildTree(int[] inorder, int[] postorder) {
        if (inorder == null || postorder == null
                || inorder.length != postorder.length
                || inorder.length < 1) {
            return null;
        }
        return buildTree(inorder, 0, inorder.length - 1, postorder, 0, postorder.length - 1);
    }

    TreeNode buildTree(int[] inorder, int p, int q, int[] postorder, int x, int y) {
        if (p > q || x > y) {
            return null;
        }
        TreeNode root = new TreeNode(postorder[y]);
        int i;
        for (i = p; i <= q; i++) {
            if (inorder[i] == root.val) {
                break;
            }
        }
        root.left = buildTree(inorder, p, i - 1, postorder, x, x + (i - p) - 1);
        root.right = buildTree(inorder, i + 1, q, postorder, x + (i - p), y - 1);
        return root;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        int[] inorder = new int[]{9, 3, 15, 20, 7};
        int[] postorder = new int[]{9, 15, 7, 20, 3};
        TreeNode root = solution.buildTree(inorder, postorder);
        TreeNode.printTree(root);
    }
}
