import java.util.Arrays;
import java.util.LinkedList;

public class Solution {

    public String serialize(TreeNode root) {
        if (root == null) {
            return "";
        }
        StringBuilder sb = new StringBuilder();
        serialize(root, sb);
        return sb.toString();
    }

    void serialize(TreeNode node, StringBuilder sb) {
        if (node == null) {
            sb.append("x ");
            return;
        }
        sb.append(node.val).append(" ");
        serialize(node.left, sb);
        serialize(node.right, sb);
    }

    public TreeNode deserialize(String data) {
        if (data == null || data.isEmpty()) {
            return null;
        }
        LinkedList<String> vals = Arrays.stream(data.trim().split(" ")).collect(LinkedList::new, LinkedList::offer, LinkedList::addAll);
        return deserialize(vals);
    }

    TreeNode deserialize(LinkedList<String> vals) {
        if (vals.size() == 0) {
            return null;
        }
        String val = vals.poll();
        if (val.equals("x")) {
            return null;
        }
        TreeNode node = new TreeNode(Integer.parseInt(val));
        node.left = deserialize(vals);
        node.right = deserialize(vals);
        return node;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        TreeNode root = TreeNode.createTree(new Integer[]{1, 2, 3, null, null, 4, 5});
        String data = solution.serialize(root);
        System.out.println(data);
        root = solution.deserialize(data);
        TreeNode.printTree(root);
    }
}
