import java.util.*;

public class TreeNode {

    final int val;
    TreeNode left;
    TreeNode right;

    TreeNode(int val) {
        this.val = val;
    }

    static TreeNode createTree(Integer[] vals) {
        if (vals == null || vals.length < 1) {
            return null;
        }
        TreeNode root = new TreeNode(vals[0]);
        List<TreeNode> currLevel = Collections.singletonList(root);
        for (int i = 1; i < vals.length; ) {
            List<TreeNode> nextLevel = new ArrayList<>();
            for (TreeNode node : currLevel) {
                Integer leftVal = vals[i++];
                if (leftVal != null) {
                    node.left = new TreeNode(leftVal);
                    nextLevel.add(node.left);
                }
                Integer rightVal = vals[i++];
                if (rightVal != null) {
                    node.right = new TreeNode(rightVal);
                    nextLevel.add(node.right);
                }
                if (i >= vals.length) {
                    break;
                }
            }
            currLevel = nextLevel;
        }
        return root;
    }

    @SuppressWarnings("unused")
    static void printTree(TreeNode root) {
        if (root == null) {
            System.out.println("null");
            return;
        }
        Queue<TreeNode> queue = new LinkedList<>();
        queue.offer(root);
        TreeNode node;
        while ((node = queue.poll()) != null) {
            System.out.print(node.val);
            System.out.print(' ');
            if (node.left == null) {
                System.out.print("null ");
            } else {
                queue.offer(node.left);
            }
            if (node.right == null) {
                System.out.print("null ");
            } else {
                queue.offer(node.right);
            }
        }
    }

    @Override
    public String toString() {
        return "TreeNode{" +
                "val=" + val +
                '}';
    }
}
