import java.util.ArrayList;
import java.util.List;

public class Solution {

    public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
        if (root == null || p == null || q == null) {
            return null;
        }
        List<TreeNode> pathToP = new ArrayList<>();
        pathToP.add(root);
        if (!findPath(root, p, pathToP)) {
            return null;
        }
        List<TreeNode> pathToQ = new ArrayList<>();
        pathToQ.add(root);
        if (!findPath(root, q, pathToQ)) {
            return null;
        }
        if (pathToP.isEmpty() || pathToQ.isEmpty()) {
            return null;
        }
        for (int i = pathToP.size() - 1; i >= 0; i--) {
            TreeNode node = pathToP.get(i);
            for (int j = pathToQ.size() - 1; j >= 0; j--) {
                if (node == pathToQ.get(j)) {
                    return node;
                }
            }
        }
        return null;
    }

    private boolean findPath(TreeNode start, TreeNode node, List<TreeNode> path) {
        if (start.val == node.val) {
            return true;
        }
        if (start.left != null) {
            path.add(start.left);
            if (findPath(start.left, node, path)) {
                return true;
            }
            path.remove(path.size() - 1);
        }
        if (start.right != null) {
            path.add(start.right);
            if (findPath(start.right, node, path)) {
                return true;
            }
            path.remove(path.size() - 1);
        }
        return false;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        Integer[] vals;
        TreeNode root;
        TreeNode ancestor;

        vals = new Integer[]{3, 5, 1, 6, 2, 0, 8, null, null, 7, 4};
        root = TreeNode.createTree(vals);
        ancestor = solution.lowestCommonAncestor(root, new TreeNode(5), new TreeNode(1));
        System.out.println(ancestor.val);
        ancestor = solution.lowestCommonAncestor(root, new TreeNode(5), new TreeNode(4));
        System.out.println(ancestor.val);

        vals = new Integer[]{3, 5, null};
        root = TreeNode.createTree(vals);
        ancestor = solution.lowestCommonAncestor(root, new TreeNode(5), new TreeNode(3));
        System.out.println(ancestor.val);
        ancestor = solution.lowestCommonAncestor(root, new TreeNode(5), new TreeNode(5));
        System.out.println(ancestor.val);
    }
}
