import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

public class TreeNode {

    int val;
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
        List<TreeNode> currLevel = new ArrayList<>();
        currLevel.add(root);
        while (true) {
            System.out.println(currLevel);

            List<TreeNode> nextLevel = new ArrayList<>();
            for (TreeNode node : currLevel) {
                if (node != null) {
                    nextLevel.add(node.left);
                    nextLevel.add(node.right);
                }
            }
            if (nextLevel.stream().noneMatch(Objects::nonNull)) {
                break;
            }
            currLevel = nextLevel;
        }
    }

    @Override
    public String toString() {
        return String.valueOf(val);
    }
}
