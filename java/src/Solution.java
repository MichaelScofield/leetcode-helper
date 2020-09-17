public class Solution {

    public int reversePairs(int[] nums) {
        if (nums == null || nums.length <= 1) {
            return 0;
        }
        return reversePairs(nums, 0, nums.length);
    }

    int reversePairs(int[] nums, int start, int end) {
        int len = end - start;
        if (len <= 1) {
            return 0;
        }
        int mid = start + len / 2;
        int leftPairs = reversePairs(nums, start, mid);
        int rightPairs = reversePairs(nums, mid, end);

        int pairs = 0;
        for (int i = mid - 1; i >= start; i--) {
            for (int j = end - 1; j >= mid; j--) {
                if ((long) nums[i] > 2 * (long) nums[j]) {
                    pairs += j - mid + 1;
                    break;
                }
            }
        }

        int[] tmp = new int[len];
        int i = mid - 1;
        int j = end - 1;
        int k = len - 1;
        while (i >= start && j >= mid) {
            if (nums[i] > nums[j]) {
                tmp[k--] = nums[i--];
            } else {
                tmp[k--] = nums[j--];
            }
        }
        while (i >= start) {
            tmp[k--] = nums[i--];
        }
        while (j >= mid) {
            tmp[k--] = nums[j--];
        }
        System.arraycopy(tmp, 0, nums, start, len);
        return pairs + leftPairs + rightPairs;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        System.out.println(0 == solution.reversePairs(new int[]{2147483647, 2147483647, 2147483647, 2147483647, 2147483647, 2147483647}));
        System.out.println(4 == solution.reversePairs(new int[]{5, 4, 3, 2, 1}));
        System.out.println(2 == solution.reversePairs(new int[]{1, 3, 2, 3, 1}));
        System.out.println(3 == solution.reversePairs(new int[]{2, 4, 3, 5, 1}));
    }
}
