import java.util.Arrays;
import java.util.List;

public class Solution {

    boolean isValid = true;

    public boolean isValidBST(TreeNode root) {
        if (root == null) {
            throw new IllegalStateException("undefined");
        }
        inorder(root, null);
        return isValid;
    }

    TreeNode inorder(TreeNode node, TreeNode pre) {
        if (!isValid) {
            return null;
        }

        if (node.left != null) {
            pre = inorder(node.left, pre);
        }

        if (pre != null) {
            if (pre.val >= node.val) {
                isValid = false;
                return null;
            }
        }

        if (node.right != null) {
            return inorder(node.right, node);
        } else {
            return node;
        }
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
        List<Boolean> expects = Arrays.asList(
                true
                , true
                , false
                , false
                , false
                , false
        );
        for (int i = 0; i < inputs.size(); i++) {
            Integer[] vals = inputs.get(i);
            TreeNode root = TreeNode.createTree(vals);
            boolean expect = expects.get(i);
            System.out.println(solution.isValidBST(root) == expect);
        }
    }
}
