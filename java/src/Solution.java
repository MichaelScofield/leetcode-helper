import java.util.Arrays;
import java.util.List;

public class Solution {

    public ListNode deleteDuplicates(ListNode head) {
        if (head == null) {
            return null;
        }
        ListNode newHead = null;
        ListNode last = head, curr = head;
        while (curr != null && curr.next != null) {
            if (curr.val == curr.next.val) {
                ListNode nextNotDuplicate = findNextNotDuplicate(curr);
                last.next = curr = nextNotDuplicate;
            } else {
                if (newHead == null) {
                    newHead = curr;
                }
                last = curr;
                curr = curr.next;
            }
        }
        return newHead == null ? curr : newHead;
    }

    ListNode findNextNotDuplicate(ListNode head) {
        ListNode curr = head;
        while (curr != null) {
            if (curr.next == null) {
                return null;
            }
            if (curr.val != curr.next.val) {
                return curr.next;
            }
            curr = curr.next;
        }
        return null;
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
            head = solution.deleteDuplicates(head);
            ListNode.printList(head);
        }
    }
}
