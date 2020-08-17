import java.util.Arrays;
import java.util.List;

public class Solution {

    public ListNode reverseList(ListNode head) {
        if (head == null) {
            return null;
        }
        ListNode prev = head;
        ListNode curr = head.next;
        head.next = null;
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
        List<Integer[]> inputs = Arrays.asList(
                new Integer[]{1, 1, 2}
                , new Integer[]{1, 1, 2, 3, 3}
                , new Integer[]{1}
                , new Integer[]{1, 1}
                , new Integer[]{1, 2, 3, 3, 4, 4, 5}
                , new Integer[]{1, 1, 1, 2, 3}
        );
        for (Integer[] vals : inputs) {
            ListNode head = ListNode.from(vals);
            head = solution.reverseList(head);
            ListNode.printList(head);
        }
    }
}
