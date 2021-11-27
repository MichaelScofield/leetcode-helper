public class Solution {

    public ListNode partition(ListNode head, int x) {
        if (head == null || head.next == null) {
            return head;
        }
        ListNode dummy = new ListNode();
        dummy.next = head;
        ListNode smaller = dummy, prev = null, curr = dummy.next;
        while (curr != null) {
            if (curr.val < x) {
                ListNode smaller_next = smaller.next;
                ListNode next = curr.next;

                if (smaller_next == curr) {
                    smaller = curr;
                } else {
                    smaller.next = curr;
                    curr.next = smaller_next;
                    smaller = smaller.next;

                    prev.next = next;
                }
                curr = next;
            } else {
                prev = curr;
                curr = curr.next;
            }
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        ListNode expected;
        ListNode actual;

        expected = ListNode.from(new Integer[]{1, 2, 3}, null);
        actual = solution.partition(ListNode.from(new Integer[]{1, 2, 3}, null), 1);
        System.out.println(ListNode.isEqual(expected, actual));

        expected = ListNode.from(new Integer[]{1, 2, 3}, null);
        actual = solution.partition(ListNode.from(new Integer[]{1, 2, 3}, null), 3);
        System.out.println(ListNode.isEqual(expected, actual));

        expected = ListNode.from(new Integer[]{1, 2, 2, 4, 3, 5}, null);
        actual = solution.partition(ListNode.from(new Integer[]{1, 4, 3, 2, 5, 2}, null), 3);
        System.out.println(ListNode.isEqual(expected, actual));

        expected = ListNode.from(new Integer[]{1, 2}, null);
        actual = solution.partition(ListNode.from(new Integer[]{2, 1}, null), 2);
        System.out.println(ListNode.isEqual(expected, actual));
    }
}
