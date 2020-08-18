import java.util.Arrays;
import java.util.List;

public class Solution {

    public ListNode sortList(ListNode head) {
        if (head == null) {
            return null;
        }
        return sort(head, null);
    }

    static class Partition {
        final ListNode subListHead;
        final ListNode pivotHead;
        final ListNode subListTail;

        Partition(ListNode subListHead, ListNode pivotHead, ListNode subListTail) {
            this.subListHead = subListHead;
            this.pivotHead = pivotHead;
            this.subListTail = subListTail;
        }

        @Override
        public String toString() {
            return "Partition{" +
                    "subListHead=" + subListHead +
                    ", pivotHead=" + pivotHead +
                    ", subListTail=" + subListTail +
                    '}';
        }
    }

    ListNode sort(ListNode head, ListNode tailNext) {
        if (head == tailNext || head.next == tailNext) {
            return head;
        }
        Partition partition = partition(head, tailNext);
        partition.subListTail.next = tailNext;
        if (partition.subListHead == partition.pivotHead) {
            partition.subListHead.next = sort(partition.pivotHead.next, tailNext);
            return partition.subListHead;
        } else {
            sort(partition.pivotHead, tailNext);
            return sort(partition.subListHead, partition.pivotHead);
        }
    }

    Partition partition(ListNode head, ListNode tailNext) {
        int pivot = head.val;
        ListNode subListDummyHead = new ListNode(0, head);
        ListNode subListHead = subListDummyHead;
        ListNode pivotDummyHead = new ListNode(0, head);
        ListNode pivotHead = pivotDummyHead;
        while (head != tailNext) {
            if (head.val < pivot) {
                subListHead.next = head;
                subListHead = subListHead.next;
            } else {
                pivotHead.next = head;
                pivotHead = pivotHead.next;
            }
            head = head.next;
        }
        subListHead.next = pivotDummyHead.next;
        return new Partition(subListDummyHead.next, pivotDummyHead.next, pivotHead);
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        class TestCase {
            final ListNode list;

            TestCase(Integer[] list) {
                this.list = ListNode.from(list);
            }
        }
        List<TestCase> inputs = Arrays.asList(
                new TestCase(new Integer[]{3, 2, 1}),
                new TestCase(new Integer[]{1, 2, 3, 4, 5}),
                new TestCase(new Integer[]{1, 4, 3, 2, 5, 2}),
                new TestCase(new Integer[]{-1, 5, 3, 4, 0}),
                new TestCase(new Integer[]{4, 2, 1, 3}),
                new TestCase(new Integer[]{1, 2}),
                new TestCase(new Integer[]{1})
        );
        for (TestCase testCase : inputs) {
            ListNode head = solution.sortList(testCase.list);
            ListNode.printList(head);
        }
    }
}
