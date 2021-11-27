public class Solution {

    public ListNode rotateRight(ListNode head, int k) {
        int len = 0;
        ListNode curr = head;
        while (curr != null) {
            len++;
            curr = curr.next;
        }
        if (len <= 1) {
            return head;
        }

        int r = k % len;
        ListNode slow = head, fast = head;
        for (int i = 0; i < r; i++) {
            fast = fast.next;
        }
        while (fast.next != null) {
            slow = slow.next;
            fast = fast.next;
        }

        fast.next = head;
        head = slow.next;
        slow.next = null;
        return head;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        ListNode expected;
        ListNode actual;

        expected = ListNode.from(new Integer[]{4, 5, 1, 2, 3}, null);
        actual = solution.rotateRight(ListNode.from(new Integer[]{1, 2, 3, 4, 5}, null), 2);
        System.out.println(ListNode.isEqual(expected, actual));

        expected = ListNode.from(new Integer[]{2, 0, 1}, null);
        actual = solution.rotateRight(ListNode.from(new Integer[]{0, 1, 2}, null), 4);
        System.out.println(ListNode.isEqual(expected, actual));
    }
}
