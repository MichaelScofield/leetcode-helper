public class Solution {

    public Node connect(Node root) {
        if (root == null) {
            return null;
        }
        connect(root.left, root.right);
        return root;
    }

    void connect(Node n1, Node n2) {
        if (n1 == null || n2 == null) {
            return;
        }
        n1.next = n2;
        connect(n1.left, n1.right);
        connect(n2.left, n2.right);
        connect(n1.right, n2.left);
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        Node root = Node.createTree(new Integer[]{1, 2, 3, 4, 5, 6, 7});
        Node.printTree(solution.connect(root));
    }
}
