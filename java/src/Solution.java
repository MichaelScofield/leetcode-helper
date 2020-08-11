import java.util.*;

public class Solution {

    public List<List<Integer>> zigzagLevelOrder(TreeNode root) {
        List<List<Integer>> levels = new ArrayList<>();
        if (root == null) {
            return levels;
        }
        Queue<TreeNode> queue = new LinkedList<>();
        queue.add(root);
        int currChildren = 1;
        boolean isZig = true;
        while (!queue.isEmpty()) {
            int nextChildren = 0;
            List<Integer> level = new ArrayList<>(currChildren);
            for (int i = 0; i < currChildren; i++) {
                TreeNode node = queue.poll();
                assert node != null;
                level.add(node.val);
                if (node.left != null) {
                    nextChildren += 1;
                    queue.add(node.left);
                }
                if (node.right != null) {
                    nextChildren += 1;
                    queue.add(node.right);
                }
            }
            currChildren = nextChildren;
            if (!isZig) {
                Collections.reverse(level);
                isZig = true;
            } else {
                isZig = false;
            }
            levels.add(level);
        }
        return levels;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        Integer[] vals;
        TreeNode root;
        List<List<Integer>> levelOrder;

        vals = new Integer[]{3, 9, 20, null, null, 15, 7};
        root = TreeNode.createTree(vals);
        levelOrder = solution.zigzagLevelOrder(root);
        System.out.println(levelOrder);

        vals = new Integer[]{3, 5, null};
        root = TreeNode.createTree(vals);
        levelOrder = solution.zigzagLevelOrder(root);
        System.out.println(levelOrder);

        vals = new Integer[]{1, null, 2, null, 3, null, 4};
        root = TreeNode.createTree(vals);
        levelOrder = solution.zigzagLevelOrder(root);
        System.out.println(levelOrder);
    }
}
