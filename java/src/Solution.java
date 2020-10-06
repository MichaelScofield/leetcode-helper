public class Solution {

    public TreeNode constructMaximumBinaryTree(int[] nums) {
        if (nums == null || nums.length == 0) {
            return null;
        }
        return constructMaximumBinaryTree(nums, 0, nums.length);
    }

    TreeNode constructMaximumBinaryTree(int[] nums, int l, int r) {
        if (l >= r) {
            return null;
        }
        int max = nums[l];
        int p = l;
        for (int i = l + 1; i < r; i++) {
            if (nums[i] > max) {
                max = nums[i];
                p = i;
            }
        }
        TreeNode root = new TreeNode(max);
        root.left = constructMaximumBinaryTree(nums, l, p);
        root.right = constructMaximumBinaryTree(nums, p + 1, r);
        return root;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        TreeNode.printTree(solution.constructMaximumBinaryTree(new int[]{3, 2, 1, 6, 0, 5}));
    }
}
