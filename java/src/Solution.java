import java.util.Arrays;
import java.util.List;

public class Solution {

    public TreeNode insertIntoBST(TreeNode root, int val) {
        if (root == null) {
            return new TreeNode(val);
        }
        assert val != root.val;
        if (val < root.val) {
            if (root.left == null) {
                root.left = new TreeNode(val);
            } else {
                insertIntoBST(root.left, val);
            }
        } else {
            if (root.right == null) {
                root.right = new TreeNode(val);
            } else {
                insertIntoBST(root.right, val);
            }
        }
        return root;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        List<Integer[]> inputs = Arrays.asList(
                new Integer[]{2, 1, 3}
                , new Integer[]{4, 2, 7, 1, 3}
        );
        for (Integer[] vals : inputs) {
            TreeNode root = TreeNode.createTree(vals);
            root = solution.insertIntoBST(root, 5);
            TreeNode.printTree(root);
            System.out.println();
        }
    }
}
