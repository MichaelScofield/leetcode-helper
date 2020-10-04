public class Solution {

    public ListNode reverseKGroup(ListNode head, int k) {
        if (head == null) {
            return null;
        }
        ListNode newHead = null;
        ListNode lastTail = null;
        ListNode subListHead = head;
        while (subListHead != null) {
            ListNode subListTail = subListHead;
            for (int i = 0; i < k; i++) {
                if (subListTail == null) {
                    return newHead;
                }
                subListTail = subListTail.next;
            }
            ListNode reversedHead = reverse(subListHead, k);
            if (newHead == null) {
                newHead = reversedHead;
            }
            if (lastTail != null) {
                lastTail.next = reversedHead;
            }
            lastTail = subListHead;
            lastTail.next = subListTail;
            subListHead = subListTail;
        }
        return newHead;
    }

    ListNode reverse(ListNode node, int len) {
        assert node != null;
        ListNode prev = null;
        ListNode curr = node;
        int i = 0;
        while (curr != null && i++ < len) {
            ListNode next = curr.next;
            curr.next = prev;
            prev = curr;
            curr = next;
        }
        node.next = curr;
        return prev;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        ListNode head;
        head = ListNode.from(new Integer[]{1, 2, 3, 4, 5}, null);
        ListNode.printList(solution.reverseKGroup(head, 2));
        head = ListNode.from(new Integer[]{1, 2, 3, 4, 5}, null);
        ListNode.printList(solution.reverseKGroup(head, 3));
        head = ListNode.from(new Integer[]{1, 2, 3, 4, 5, 6}, null);
        ListNode.printList(solution.reverseKGroup(head, 3));
    }
}
