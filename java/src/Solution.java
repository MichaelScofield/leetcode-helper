import java.util.Deque;
import java.util.LinkedList;

class BSTIterator {

    TreeNode node;
    Deque<TreeNode> stack = new LinkedList<>();

    public BSTIterator(TreeNode root) {
        node = root;
    }

    public int next() {
        while (node != null) {
            stack.push(node);
            node = node.left;
        }
        node = stack.pop();
        int next = node.val;
        node = node.right;
        return next;
    }

    public boolean hasNext() {
        return node != null || !stack.isEmpty();
    }
}

public class Solution {

    public static void main(String[] args) {
        TreeNode root = TreeNode.createTree(new Integer[]{7, 3, 15, null, null, 9, 20});
        BSTIterator it = new BSTIterator(root);
        System.out.println(it.next());
        System.out.println(it.next());
        System.out.println(it.hasNext());
        System.out.println(it.next());
        System.out.println(it.hasNext());
        System.out.println(it.next());
        System.out.println(it.hasNext());
        System.out.println(it.next());
        System.out.println(it.hasNext());
    }
}
