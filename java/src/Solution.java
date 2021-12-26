import java.util.*;

// Definition for a Node.
class Node {
    public int val;
    public List<Node> neighbors;

    public Node() {
        val = 0;
        neighbors = new ArrayList<>();
    }

    public Node(int _val) {
        val = _val;
        neighbors = new ArrayList<>();
    }

    public Node(int _val, ArrayList<Node> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
}

public class Solution {

    static class NodePair {
        final Node curr;
        final Node currNew;

        NodePair(Node curr, Node currNew) {
            this.curr = curr;
            this.currNew = currNew;
        }
    }

    Map<Integer, Node> nodes = new HashMap<>();

    public Node cloneGraph(Node node) {
        if (node == null) {
            return null;
        }
        Node newNode = new Node();
        LinkedList<NodePair> queue = new LinkedList<>();
        queue.push(new NodePair(node, newNode));
        while (!queue.isEmpty()) {
            NodePair nodePair = queue.pop();
            Node curr = nodePair.curr;

            List<Node> neighbors = curr.neighbors;
            ArrayList<Node> newNeighbors = new ArrayList<>(neighbors.size());
            for (Node neighbor : neighbors) {
                if (nodes.containsKey(neighbor.val)) {
                    newNeighbors.add(nodes.get(neighbor.val));
                } else {
                    Node newNeighbor = new Node(neighbor.val);
                    newNeighbors.add(newNeighbor);

                    nodes.put(neighbor.val, newNeighbor);
                    queue.push(new NodePair(neighbor, newNeighbor));
                }
            }

            Node currNew = nodePair.currNew;
            currNew.val = curr.val;
            currNew.neighbors = newNeighbors;
            nodes.put(currNew.val, currNew);
        }
        return newNode;
    }

    public static void main(String[] args) {
        Solution solution;

        Node n1 = new Node(1);
        Node n2 = new Node(2);
        Node n3 = new Node(3);
        Node n4 = new Node(4);
        n1.neighbors = Arrays.asList(n2, n4);
        n2.neighbors = Arrays.asList(n1, n3);
        n3.neighbors = Arrays.asList(n2, n4);
        n4.neighbors = Arrays.asList(n1, n3);
        print(n1);
        solution = new Solution();
        Node newN1 = solution.cloneGraph(n1);
        print(newN1);

        Node n5 = new Node(1);
        print(n5);
        solution = new Solution();
        Node newN5 = solution.cloneGraph(n5);
        print(newN5);

        Node n6 = new Node(1);
        Node n7 = new Node(2);
        n6.neighbors = List.of(n7);
        n7.neighbors = List.of(n6);
        print(n6);
        solution = new Solution();
        Node newN6 = solution.cloneGraph(n6);
        print(newN6);
    }

    static void print(Node node) {
        LinkedList<Node> queue = new LinkedList<>();
        queue.push(node);
        Set<Integer> is_printed = new HashSet<>();
        while (!queue.isEmpty()) {
            Node n = queue.pop();
            if (!is_printed.add(n.val)) {
                continue;
            }
            System.out.println(n.val + " -> " + Arrays.toString(
                    n.neighbors.stream().map(x -> x.val).toArray()));
            queue.addAll(n.neighbors);
        }
        System.out.println();
    }
}
