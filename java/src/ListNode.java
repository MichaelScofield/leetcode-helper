public class ListNode {

    int val;
    ListNode next;

    ListNode() {
        // empty
    }

    ListNode(int val) {
        this.val = val;
    }

    ListNode(int val, ListNode next) {
        this.val = val;
        this.next = next;
    }

    static ListNode from(Integer[] vals, Integer cyclePos) {
        ListNode head = null;
        ListNode curr = null;
        ListNode cycleNode = null;
        for (int i = 0; i < vals.length; i++) {
            int val = vals[i];
            ListNode node = new ListNode(val);
            if (cyclePos != null && cycleNode == null && i == cyclePos) {
                cycleNode = node;
            }
            if (head == null) {
                head = curr = node;
                continue;
            }
            curr.next = node;
            curr = node;
        }
        if (cycleNode != null) {
            curr.next = cycleNode;
        }
        return head;
    }

    static void printList(ListNode head) {
        if (head == null) {
            System.out.println("null");
            return;
        }
        ListNode curr = head;
        while (!Thread.currentThread().isInterrupted()) {
            System.out.print(curr.val);
            if (curr.next == null) {
                break;
            }
            System.out.print("->");
            curr = curr.next;
        }
        System.out.println();
    }

    static boolean isEqual(ListNode l1, ListNode l2) {
        ListNode h1 = l1;
        ListNode h2 = l2;
        while (h1 != null && h2 != null) {
            if (h1.val != h2.val) {
                return false;
            }
            h1 = h1.next;
            h2 = h2.next;
        }
        return h1 == null && h2 == null;
    }

    @Override
    public String toString() {
        return String.valueOf(val);
    }
}
