public class Solution {

    public ListNode mergeTwoLists(ListNode l1, ListNode l2) {
        if (l1 == null) {
            return l2;
        }
        if (l2 == null) {
            return l1;
        }
        ListNode dummyHead = new ListNode(0);
        ListNode curr = dummyHead;
        while (l1 != null && l2 != null) {
            if (l1.val < l2.val) {
                curr.next = l1;
                l1 = l1.next;
            } else {
                curr.next = l2;
                l2 = l2.next;
            }
            curr = curr.next;
        }
        if (l1 != null) {
            curr.next = l1;
        } else {
            curr.next = l2;
        }
        return dummyHead.next;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        ListNode l1 = ListNode.from(new Integer[]{1, 2, 4}, null);
        ListNode l2 = ListNode.from(new Integer[]{1, 3, 4}, null);
        ListNode.printList(solution.mergeTwoLists(l1, l2));
    }
}
