public class Solution {

    public ListNode removeNthFromEnd(ListNode head, int n) {
        if (head == null) {
            return null;
        }
        if (n <= 0) {
            return head;
        }
        ListNode fast = head, slow = head;
        for (int i = 0; i < n - 1; i++) {
            if (fast.next != null) {
                fast = fast.next;
            } else {
                return head;
            }
        }
        ListNode prev = null;
        while (fast.next != null) {
            fast = fast.next;
            prev = slow;
            slow = slow.next;
        }
        if (prev == null) {
            return head.next;
        } else {
            prev.next = slow.next;
            return head;
        }
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        ListNode head;

        head = ListNode.from(new Integer[]{1, 2, 3, 4, 5});
        head = solution.removeNthFromEnd(head, 2);
        ListNode.printList(head);

        head = ListNode.from(new Integer[]{1, 2, 3, 4, 5});
        head = solution.removeNthFromEnd(head, 1);
        ListNode.printList(head);

        head = ListNode.from(new Integer[]{1, 2, 3, 4, 5});
        head = solution.removeNthFromEnd(head, 5);
        ListNode.printList(head);

        head = ListNode.from(new Integer[]{1, 2, 3, 4, 5});
        head = solution.removeNthFromEnd(head, 6);
        ListNode.printList(head);
    }
}
