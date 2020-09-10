public class Solution {

    TreeNode lastNode;

    public TreeNode treeToDoublyList(TreeNode root) {
        if (root == null) {
            return null;
        }
        TreeNode head = root;
        TreeNode tail = root;
        convert(root);
        while (head.left != null) {
            head = head.left;
        }
        while (tail.right != null) {
            tail = tail.right;
        }
        head.left = tail;
        tail.right = head;
        return head;
    }

    void convert(TreeNode node) {
        if (node.left != null) {
            convert(node.left);
        }
        if (lastNode != null) {
            node.left = lastNode;
            lastNode.right = node;
        }
        lastNode = node;
        if (node.right != null) {
            convert(node.right);
        }
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        TreeNode root = TreeNode.createTree(new Integer[]{4, 2, 5, 1, 3});
        TreeNode head = solution.treeToDoublyList(root);
        TreeNode last = head.left;
        TreeNode curr = head;
        do {
            System.out.println(curr.val);
            System.out.println(curr.left == last);
            last = curr;
            curr = curr.right;
        } while (last.right != head);
    }
}
