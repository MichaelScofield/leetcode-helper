public class Solution {

    static class Node {
        int val;
        Node next;
        Node random;

        public Node(int val) {
            this.val = val;
            this.next = null;
            this.random = null;
        }
    }

    public Node copyRandomList(Node head) {
        if (head == null) {
            return null;
        }
        linkedClone(head);
        linkRandoms(head);
        return splitList(head);
    }

    void linkedClone(Node head) {
        while (head != null) {
            Node node = new Node(head.val);
            node.next = head.next;
            head.next = node;
            head = node.next;
        }
    }

    void linkRandoms(Node head) {
        while (head != null) {
            if (head.random != null) {
                head.next.random = head.random.next;
            }
            head = head.next.next;
        }
    }

    Node splitList(Node head) {
        Node dummy1 = new Node(0);
        Node dummy2 = new Node(0);
        Node newHead = dummy2;
        int i = 0;
        while (head != null) {
            if (i++ % 2 == 0) {
                dummy1.next = head;
                dummy1 = head;
            } else {
                dummy2.next = head;
                dummy2 = head;
            }
            head = head.next;
        }
        dummy1.next = null;
        return newHead.next;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();

        Node n1 = new Node(7);
        Node n2 = new Node(13);
        Node n3 = new Node(11);
        Node n4 = new Node(10);
        Node n5 = new Node(1);
        n1.next = n2;
        n2.next = n3;
        n3.next = n4;
        n4.next = n5;
        n2.random = n1;
        n3.random = n5;
        n4.random = n3;
        n5.random = n1;

        Node head = n1;
        Node newHead = solution.copyRandomList(n1);
        StringBuilder sb1 = new StringBuilder();
        StringBuilder sb2 = new StringBuilder();
        while (head != null && newHead != null) {
            sb1.append("(").append(head.val).append(",").append(head.random == null ? "null" : head.random.val).append(") -> ");
            sb2.append("(").append(newHead.val).append(",").append(newHead.random == null ? "null" : newHead.random.val).append(") -> ");
            head = head.next;
            newHead = newHead.next;
        }
        System.out.println(sb1.toString());
        System.out.println(sb2.toString());
    }
}
