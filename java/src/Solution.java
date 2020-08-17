import java.util.Arrays;
import java.util.List;

public class Solution {

    public ListNode partition(ListNode head, int x) {
        if (head == null) {
            return null;
        }
        ListNode prev = null, curr = head;
        ListNode less = null;
        while (curr != null) {
            ListNode next = curr.next;
            if (curr.val < x) {
                if (prev != null) {
                    prev.next = next;
                }
                if (less == null) {
                    less = curr;
                    less.next = head;
                    head = less;
                } else {
                    if (less != curr && less.next != curr) {
                        ListNode lessNext = less.next;
                        less.next = curr;
                        curr.next = lessNext;
                    }
                    less = curr;
                }
            } else {
                prev = curr;
            }
            curr = next;
        }
        return head;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        class TestCase {
            final ListNode list;
            final int x;

            TestCase(Integer[] list, int x) {
                this.list = ListNode.from(list);
                this.x = x;
            }
        }
        List<TestCase> inputs = Arrays.asList(
                new TestCase(new Integer[]{1, 4, 3, 2, 5, 2}, 3)
                , new TestCase(new Integer[]{1, 2, 3, 4, 5}, 3)
                , new TestCase(new Integer[]{1}, 1)
                , new TestCase(new Integer[]{1, 2}, 2)
                , new TestCase(new Integer[]{2, 3}, 2)
                , new TestCase(new Integer[]{2, 1}, 2)
                , new TestCase(new Integer[]{3, 2, 1}, 2)
        );
        for (TestCase testCase : inputs) {
            ListNode head = solution.partition(testCase.list, testCase.x);
            ListNode.printList(head);
        }
    }
}
