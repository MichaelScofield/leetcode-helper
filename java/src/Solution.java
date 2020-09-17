import java.util.concurrent.atomic.AtomicInteger;

public class Solution {

    public boolean isBalanced(TreeNode root) {
        return isBalanced(root, new AtomicInteger(0));
    }

    boolean isBalanced(TreeNode root, AtomicInteger depth) {
        if (root == null) {
            depth.set(0);
            return true;
        }
        AtomicInteger leftDepth = new AtomicInteger(0);
        AtomicInteger rightDepth = new AtomicInteger(0);
        if (isBalanced(root.left, leftDepth) && isBalanced(root.right, rightDepth)) {
            if (Math.abs(leftDepth.get() - rightDepth.get()) <= 1) {
                depth.set(1 + Math.max(leftDepth.get(), rightDepth.get()));
                return true;
            }
        }
        return false;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        TreeNode root;
        root = TreeNode.createTree(new Integer[]{3, 9, 20, null, null, 15, 7});
        System.out.println(solution.isBalanced(root));
        root = TreeNode.createTree(new Integer[]{1, 2, 2, 3, 3, null, null, 4, 4});
        System.out.println(solution.isBalanced(root));
    }
}
