import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

public class Node {

    int val;
    Node left;
    Node right;
    Node next;

    Node(int val) {
        this.val = val;
    }

    static Node createTree(Integer[] vals) {
        if (vals == null || vals.length < 1) {
            return null;
        }
        Node root = new Node(vals[0]);
        List<Node> currLevel = Collections.singletonList(root);
        for (int i = 1; i < vals.length; ) {
            List<Node> nextLevel = new ArrayList<>();
            for (Node node : currLevel) {
                Integer leftVal = vals[i++];
                if (leftVal != null) {
                    node.left = new Node(leftVal);
                    nextLevel.add(node.left);
                }
                Integer rightVal = vals[i++];
                if (rightVal != null) {
                    node.right = new Node(rightVal);
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
    static void printTree(Node root) {
        if (root == null) {
            System.out.println("null");
            return;
        }
        List<Node> currLevel = new ArrayList<>();
        currLevel.add(root);
        while (true) {
            System.out.println(currLevel);

            List<Node> nextLevel = new ArrayList<>();
            for (Node node : currLevel) {
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
