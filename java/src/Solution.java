public class Solution {

    int k;

    public int kthSmallest(TreeNode root, int k) {
        assert k >= 1;
        this.k = k;
        return kthSmallest(root).val;
    }

    TreeNode kthSmallest(TreeNode root) {
        if (root == null) {
            return null;
        }
        if (root.left == null && root.right == null) {
            return --k == 0 ? root : null;
        }
        if (root.left != null) {
            TreeNode node = kthSmallest(root.left);
            if (node != null) {
                return node;
            }
        }
        if (--k == 0) {
            return root;
        }
        return kthSmallest(root.right);
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        TreeNode root;
        root = TreeNode.createTree(new Integer[]{3, 1, 4, null, 2});
        System.out.println(solution.kthSmallest(root, 1)); // 1
        root = TreeNode.createTree(new Integer[]{5, 3, 6, 2, 4, null, null, 1, null});
        System.out.println(solution.kthSmallest(root, 3)); // 3
    }
}
