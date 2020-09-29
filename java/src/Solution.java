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
        for (int i = copy.length - 1; i > 0; i--) {
            int r = random.nextInt(i + 1);
            if (i != r) {
                swap(copy, i, r);
            }
        }
        return copy;
    }

    void swap(int[] nums, int i, int j) {
        int tmp = nums[i];
        nums[i] = nums[j];
        nums[j] = tmp;
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
