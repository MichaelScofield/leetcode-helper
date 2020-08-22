import java.util.Arrays;

public class Solution {

    public int findDuplicate(int[] nums) {
        if (nums == null) {
            return -1;
        }
        int i = 1, j = nums.length - 1;
        while (i != j) {
            int mid = i + (j - i) / 2;
            int c = countNumsInRange(nums, i, mid);
            int expected = mid - i + 1;
            if (c <= expected) {
                i = mid + 1;
            } else {
                j = mid;
            }
        }
        return i;
    }

    private int countNumsInRange(int[] nums, int i, int j) {
        return (int) Arrays.stream(nums).filter(n -> n >= i && n <= j).count();
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        int[] nums = new int[]{1, 3, 4, 2, 2};
        int duplicate = solution.findDuplicate(nums);
        System.out.println(duplicate == 2);

        nums = new int[]{3, 1, 3, 4, 2};
        duplicate = solution.findDuplicate(nums);
        System.out.println(duplicate == 3);

        nums = new int[]{2, 2, 2, 2, 2};
        duplicate = solution.findDuplicate(nums);
        System.out.println(duplicate == 2);

        nums = new int[]{1, 1};
        duplicate = solution.findDuplicate(nums);
        System.out.println(duplicate == 1);
    }
}
