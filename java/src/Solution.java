public class Solution {

    public ListNode getIntersectionNode(ListNode headA, ListNode headB) {
        if (headA == null || headB == null) {
            return null;
        }
        int l1 = len(headA), l2 = len(headB);
        if (l1 > l2) {
            headA = forward(headA, l1 - l2);
        } else {
            headB = forward(headB, l2 - l1);
        }
        while (headA != null && headB != null) {
            if (headA == headB) {
                return headA;
            }
            headA = headA.next;
            headB = headB.next;
        }
        return null;
    }

    int len(ListNode head) {
        int l = 0;
        while (head != null) {
            l += 1;
            head = head.next;
        }
        return l;
    }

    ListNode forward(ListNode head, int n) {
        while (n-- > 0) {
            head = head.next;
        }
        return head;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();

        ListNode n1_1 = new ListNode(4);
        ListNode n1_2 = new ListNode(1);
        ListNode n1_3 = new ListNode(8);
        ListNode n1_4 = new ListNode(4);
        ListNode n1_5 = new ListNode(5);
        n1_1.next = n1_2;
        n1_2.next = n1_3;
        n1_3.next = n1_4;
        n1_4.next = n1_5;
        ListNode n2_1 = new ListNode(5);
        ListNode n2_2 = new ListNode(6);
        ListNode n2_3 = new ListNode(1);
        n2_1.next = n2_2;
        n2_2.next = n2_3;
        n2_3.next = n1_3;
        System.out.println(solution.getIntersectionNode(n1_1, n2_1) == n1_3);

        ListNode headA = ListNode.from(new Integer[]{2, 6, 4}, null);
        ListNode headB = ListNode.from(new Integer[]{1, 5}, null);
        System.out.println(solution.getIntersectionNode(headA, headB) == null);
    }
}
