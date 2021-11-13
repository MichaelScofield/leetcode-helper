public class Solution {

    public ListNode swapPairs(ListNode head) {
        ListNode dummy = new ListNode(0);
        dummy.next = head;
        ListNode prev = dummy;
        ListNode curr = head;
        while (curr != null) {
            ListNode next = curr.next;
            if (next == null) {
                break;
            }
            ListNode nextnext = next.next;
            prev.next = next;
            next.next = curr;
            curr.next = nextnext;
            prev = curr;
            curr = nextnext;
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        ListNode expected;
        ListNode actual;

        expected = ListNode.from(new Integer[]{2, 1, 4, 3, 5}, null);
        actual = solution.swapPairs(ListNode.from(new Integer[]{1, 2, 3, 4, 5}, null));
        System.out.println(ListNode.isEqual(expected, actual));

        expected = ListNode.from(new Integer[]{2, 1, 4, 3}, null);
        actual = solution.swapPairs(ListNode.from(new Integer[]{1, 2, 3, 4}, null));
        System.out.println(ListNode.isEqual(expected, actual));

        expected = ListNode.from(new Integer[]{1}, null);
        actual = solution.swapPairs(ListNode.from(new Integer[]{1}, null));
        System.out.println(ListNode.isEqual(expected, actual));

        expected = ListNode.from(new Integer[]{}, null);
        actual = solution.swapPairs(ListNode.from(new Integer[]{}, null));
        System.out.println(ListNode.isEqual(expected, actual));
    }
}
