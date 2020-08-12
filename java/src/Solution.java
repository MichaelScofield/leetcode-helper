import java.util.Arrays;
import java.util.List;

public class Solution {

    public boolean isValidBST(TreeNode root) {
        if (root == null) {
            return true;
        }
        if (isNotValidBst(root)) {
            return false;
        }
        if (root.left != null) {
            TreeNode larger = findLarger(root.left);
            if (larger == null) {
                return false;
            }
            if (root.val <= larger.val) {
                return false;
            }
        }
        if (root.right != null) {
            TreeNode smaller = findSmaller(root.right);
            if (smaller == null) {
                return false;
            }
            if (root.val >= smaller.val) {
                return false;
            }
        }
        return isValidBST(root.left) && isValidBST(root.right);
    }

    TreeNode findLarger(TreeNode node) {
        if (isNotValidBst(node)) {
            return null;
        }
        if (node.right != null) {
            return findLarger(node.right);
        }
        return node;
    }

    TreeNode findSmaller(TreeNode node) {
        if (isNotValidBst(node)) {
            return null;
        }
        if (node.left != null) {
            return findSmaller(node.left);
        }
        return node;
    }

    boolean isNotValidBst(TreeNode node) {
        return node.right != null && node.right.val <= node.val ||
                node.left != null && node.left.val >= node.val;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        List<Integer[]> inputs = Arrays.asList(
                new Integer[]{2, 1, 3}
                , new Integer[]{5, 2, 6, 1, 3}
                , new Integer[]{5, 1, 4, null, null, 3, 6}
                , new Integer[]{5, 3, 6, 1, 5}
                , new Integer[]{5, 2, 6, 1, 3, 4, null}
                , new Integer[]{3, null, 30, 10, null, null, 15, null, 45}
        );
        for (Integer[] vals : inputs) {
            TreeNode root = TreeNode.createTree(vals);
            System.out.println(solution.isValidBST(root));
        }
    }
}
