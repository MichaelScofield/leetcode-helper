import java.util.Comparator;
import java.util.PriorityQueue;

public class Solution {

    public ListNode mergeKLists(ListNode[] lists) {
        if (lists == null || lists.length == 0) {
            return null;
        }

        PriorityQueue<ListNode> minHeap = new PriorityQueue<>(Comparator.comparingInt(o -> o.val));
        for (ListNode list : lists) {
            if (list != null) {
                minHeap.add(list);
            }
        }

        ListNode dummyHead = new ListNode();
        ListNode head = dummyHead;
        while (!minHeap.isEmpty()) {
            ListNode minNode = minHeap.poll();
            ListNode node = new ListNode(minNode.val);
            dummyHead.next = node;
            dummyHead = node;

            if (minNode.next != null) {
                minHeap.add(minNode.next);
            }
        }
        return head.next;
    }

    public static void main(String[] args) {
        ListNode l1 = ListNode.from(new int[]{1, 4, 5});
        ListNode l2 = ListNode.from(new int[]{1, 3, 4});
        ListNode l3 = ListNode.from(new int[]{2, 6});
        Solution solution = new Solution();
        ListNode merged = solution.mergeKLists(new ListNode[]{l1, l2, l3});
        ListNode.printList(merged);
        System.out.println(ListNode.isEqual(
                ListNode.from(new int[]{1, 1, 2, 3, 4, 4, 5, 6}), merged));
    }
}
