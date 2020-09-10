public class Solution {

    public TreeNode deleteNode(TreeNode root, int key) {
        if (root == null) {
            return null;
        }
        if (root.val == key) {
            if (root.left == null && root.right == null) {
                return null;
            }
            if (root.left != null && root.right != null) {
                TreeNode min = root.right;
                while (min.left != null) {
                    min = min.left;
                }
                root.val = min.val;
                root.right = deleteNode(root.right, min.val);
                return root;
            }
            return root.left != null ? root.left : root.right;
        } else if (root.val < key) {
            root.right = deleteNode(root.right, key);
        } else {
            root.left = deleteNode(root.left, key);
        }
        return root;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        TreeNode root;

        root = TreeNode.createTree(new Integer[]{5, 3, 6, 2, 4, null, 7});
        root = solution.deleteNode(root, 3);
        TreeNode.printTree(root);
        System.out.println();

        root = TreeNode.createTree(new Integer[]{5, 3, 6, 2, 4, null, 7});
        root = solution.deleteNode(root, 0);
        TreeNode.printTree(root);
        System.out.println();

        root = TreeNode.createTree(new Integer[]{});
        root = solution.deleteNode(root, 0);
        TreeNode.printTree(root);
        System.out.println();

        root = TreeNode.createTree(new Integer[]{5, 3, 6, 2, 4, null, 7});
        root = solution.deleteNode(root, 5);
        TreeNode.printTree(root);
        System.out.println();

        root = TreeNode.createTree(new Integer[]{50, 30, 70, null, 40, 60, 80});
        root = solution.deleteNode(root, 50);
        TreeNode.printTree(root);
        System.out.println();

        root = TreeNode.createTree(new Integer[]{5});
        root = solution.deleteNode(root, 5);
        TreeNode.printTree(root);
        System.out.println();

        root = TreeNode.createTree(new Integer[]{1, null, 2, null, 3});
        root = solution.deleteNode(root, 2);
        TreeNode.printTree(root);
        System.out.println();
    }
}
