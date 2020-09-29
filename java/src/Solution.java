import java.util.Arrays;
import java.util.Random;

class Solution {

    final int[] nums;

    public Solution(int[] nums) {
        this.nums = nums;
    }

    public int[] reset() {
        return nums;
    }

    public int[] shuffle() {
        int[] copy = Arrays.copyOf(nums, nums.length);
        Random random = new Random(System.nanoTime());
        for (int i = 0; i < copy.length; i++) {
            int r = random.nextInt(copy.length);
            int tmp = copy[i];
            copy[i] = copy[r];
            copy[r] = tmp;
        }
        return copy;
    }

    public static void main(String[] args) {
        Solution solution = new Solution(new int[]{1, 2, 3});
        System.out.println(Arrays.toString(solution.shuffle()));
        System.out.println(Arrays.toString(solution.reset()));
        for (int i = 0; i < 10; i++) {
            System.out.println(Arrays.toString(solution.shuffle()));
        }
    }
}
