public class Solution {

    public boolean isPalindrome(ListNode head) {
        if (head == null) {
            return true;
        }
        int len = 0;
        ListNode slow = head, fast = head;
        while (fast != null) {
            fast = fast.next;
            if (fast != null) {
                fast = fast.next;
                len += 2;
            } else {
                len += 1;
            }
            slow = slow.next;
        }
        if (len == 1) {
            return true;
        }
        ListNode reverse = reverse(slow);
        while (reverse != null) {
            if (head.val != reverse.val) {
                return false;
            }
            head = head.next;
            reverse = reverse.next;
        }
        return true;
    }

    ListNode reverse(ListNode node) {
        ListNode prev = null;
        ListNode curr = node;
        while (curr != null) {
            ListNode next = curr.next;
            curr.next = prev;
            prev = curr;
            curr = next;
        }
        return prev;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        ListNode head;
        head = ListNode.from(new Integer[]{1}, null);
        System.out.println(solution.isPalindrome(head));
        head = ListNode.from(new Integer[]{1, 2}, null);
        System.out.println(!solution.isPalindrome(head));
        head = ListNode.from(new Integer[]{1, 0, 0}, null);
        System.out.println(!solution.isPalindrome(head));
        head = ListNode.from(new Integer[]{1, 2, 2, 1}, null);
        System.out.println(solution.isPalindrome(head));
    }
}
