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
            if (i == mid && c > 1) {
                break;
            }
            if (c > mid - i + 1) {
                j = mid;
            } else {
                assert c == mid - i + 1;
                i = mid + 1;
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
    }
}
