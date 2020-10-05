import java.util.Random;

public class Solution {

    final int[] nums;

    public Solution(int[] nums) {
        this.nums = nums;
    }

    public int pick(int target) {
        int pick = 0;
        int n = 1;
        Random random = new Random();
        for (int i = 0; i < nums.length; i++) {
            if (nums[i] == target && random.nextInt(n++) == 0) {
                pick = i;
            }
        }
        return pick;
    }

    public static void main(String[] args) {
        int[] nums = new int[]{1, 2, 3, 3, 3};
        Solution solution = new Solution(nums);
        for (int i = 0; i < 10; i++) {
            System.out.println(solution.pick(3));
            System.out.println(solution.pick(1));
        }
    }
}
