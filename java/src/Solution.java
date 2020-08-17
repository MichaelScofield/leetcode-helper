import java.util.Arrays;
import java.util.List;

public class Solution {

    public ListNode reverseBetween(ListNode head, int m, int n) {
        if (head == null) {
            return null;
        }
        assert m >= 1 && n >= m;
        ListNode prev = null;
        ListNode curr = head;
        for (int i = 1; i < m; i++) {
            prev = curr;
            curr = curr.next;
        }
        ListNode reverse = reverse(curr, n - m + 1);
        if (prev == null) {
            return reverse;
        }
        prev.next = reverse;
        return head;
    }

    ListNode reverse(ListNode node, int len) {
        assert node != null;
        ListNode prev = null;
        ListNode curr = node;
        int i = 0;
        while (curr != null && i++ < len) {
            ListNode next = curr.next;
            curr.next = prev;
            prev = curr;
            curr = next;
        }
        node.next = curr;
        return prev;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        class TestCase {
            final ListNode list;
            final int m;
            final int n;

            TestCase(ListNode list, int m, int n) {
                this.list = list;
                this.m = m;
                this.n = n;
            }
        }
        List<TestCase> inputs = Arrays.asList(
                new TestCase(ListNode.from(new Integer[]{1, 2, 3, 4, 5}), 2, 4)
                , new TestCase(ListNode.from(new Integer[]{1, 2, 3, 4, 5}), 1, 5)
                , new TestCase(ListNode.from(new Integer[]{1, 2, 3, 4, 5}), 1, 1)
                , new TestCase(ListNode.from(new Integer[]{1, 2, 3, 4, 5}), 3, 3)
                , new TestCase(ListNode.from(new Integer[]{1, 2, 3, 4, 5}), 4, 5)
                , new TestCase(ListNode.from(new Integer[]{1}), 1, 1)
                , new TestCase(ListNode.from(new Integer[]{1, 2}), 2, 2)
                , new TestCase(ListNode.from(new Integer[]{1, 2}), 1, 2)
        );
        for (TestCase testCase : inputs) {
            ListNode head = solution.reverseBetween(testCase.list, testCase.m, testCase.n);
            ListNode.printList(head);
        }
    }
}
