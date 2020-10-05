import java.util.Random;

public class Solution {

    final ListNode head;

    /**
     * @param head The linked list's head.
     *             Note that the head is guaranteed to be not null, so it contains at least one node.
     */
    public Solution(ListNode head) {
        this.head = head;
    }

    /**
     * Returns a random node's value.
     */
    public int getRandom() {
        int i = 1;
        int result = 0;
        Random random = new Random();
        for (ListNode curr = head; curr != null; curr = curr.next) {
            if (random.nextInt(i++) == 0) {
                result = curr.val;
            }
        }
        return result;
    }

    public static void main(String[] args) {
        ListNode head = ListNode.from(new Integer[]{1, 2, 3}, null);
        Solution solution = new Solution(head);
        for (int i = 0; i < 10; i++) {
            System.out.println(solution.getRandom());
        }
    }
}
