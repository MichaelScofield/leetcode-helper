public class Solution {

    public TreeNode buildTree(int[] preorder, int[] inorder) {
        if (preorder == null || inorder == null
                || preorder.length != inorder.length
                || preorder.length < 1) {
            return null;
        }
        return buildTree(preorder, 0, preorder.length, inorder, 0, inorder.length);
    }

    TreeNode buildTree(int[] preorder, int p, int q, int[] inorder, int x, int y) {
        if (p >= q || x >= y) {
            return null;
        }
        TreeNode root = new TreeNode(preorder[p]);
        int i;
        for (i = x; i < y; i++) {
            if (inorder[i] == preorder[p]) {
                break;
            }
        }
        root.left = buildTree(preorder, p + 1, p + 1 + (i - x), inorder, x, i);
        root.right = buildTree(preorder, p + 1 + (i - x), q, inorder, i + 1, y);
        return root;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        int[] preorder = new int[]{3, 9, 20, 15, 7};
        int[] inorder = new int[]{9, 3, 15, 20, 7};
        TreeNode root = solution.buildTree(preorder, inorder);
        TreeNode.printTree(root);
    }
}
