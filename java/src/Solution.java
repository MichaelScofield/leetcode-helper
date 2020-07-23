import java.util.Arrays;
import java.util.List;
import java.util.Objects;
import java.util.stream.Collectors;

public class Solution {

    public ListNode mergeKLists(ListNode[] lists) {
        ListNode head = null;
        ListNode curr = null;
        List<ListNode> ps = Arrays.stream(lists).filter(Objects::nonNull).collect(Collectors.toList());
        while (!ps.isEmpty()) {
            int minVal = Integer.MAX_VALUE;
            int minListIndex = -1;
            for (int i = 0; i < ps.size(); i++) {
                ListNode p = ps.get(i);
                if (p.val <= minVal) {
                    minVal = p.val;
                    minListIndex = i;
                }
            }
            ListNode node = new ListNode(minVal);
            if (head == null) {
                head = curr = node;
            } else {
                curr.next = node;
                curr = node;
            }
            ListNode pNext = ps.get(minListIndex).next;
            if (pNext == null) {
                ps.remove(minListIndex);
            } else {
                ps.set(minListIndex, pNext);
            }
        }
        return head;
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
