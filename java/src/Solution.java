import java.util.Arrays;
import java.util.List;

public class Solution {

    public ListNode deleteDuplicates(ListNode head) {
        if (head == null) {
            return null;
        }
        for (ListNode curr = head; ; ) {
            if (curr.next == null) {
                break;
            }
            if (curr.val == curr.next.val) {
                curr.next = curr.next.next;
            } else {
                curr = curr.next;
            }
        }
        return head;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        List<Integer[]> inputs = Arrays.asList(
                new Integer[]{1, 1, 2}
                , new Integer[]{1, 1, 2, 3, 3}
                , new Integer[]{1}
                , new Integer[]{1, 1}
        );
        for (Integer[] vals : inputs) {
            ListNode head = ListNode.from(vals);
            head = solution.deleteDuplicates(head);
            ListNode.printList(head);
        }
    }
}
