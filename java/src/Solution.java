public class Solution {

    public ListNode detectCycle(ListNode head) {
        if (head == null) {
            return null;
        }
        ListNode fast = head, slow = head;
        while (fast.next != null) {
            if (fast.next.next == null) {
                return null;
            }
            fast = fast.next.next;
            slow = slow.next;
            if (fast == slow) {
                break;
            }
        }
        if (fast.next == null) {
            return null;
        }
        ListNode meet = slow;
        int n = 1;
        while (slow.next != meet) {
            slow = slow.next;
            n += 1;
        }
        fast = slow = head;
        for (int i = 0; i < n; i++) {
            fast = fast.next;
        }
        while (fast != slow) {
            fast = fast.next;
            slow = slow.next;
        }
        return slow;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        ListNode head, node;

        head = ListNode.from(new Integer[]{3, 2, 0, 4}, 1);
        node = solution.detectCycle(head);
        System.out.println(node);

        head = ListNode.from(new Integer[]{3, 2, 0, 4}, 3);
        node = solution.detectCycle(head);
        System.out.println(node);

        head = ListNode.from(new Integer[]{3, 2, 0, 4}, null);
        node = solution.detectCycle(head);
        System.out.println(node);

        head = ListNode.from(new Integer[]{3, 2, 0}, null);
        node = solution.detectCycle(head);
        System.out.println(node);

        head = ListNode.from(new Integer[]{1, 2}, 0);
        node = solution.detectCycle(head);
        System.out.println(node);

        head = ListNode.from(new Integer[]{1}, null);
        node = solution.detectCycle(head);
        System.out.println(node);
    }
}
