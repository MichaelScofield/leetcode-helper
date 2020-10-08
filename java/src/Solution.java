import java.util.Collections;
import java.util.Stack;

public class Solution {

    public String removeDuplicateLetters(String s) {
        char[] chars = s.toCharArray();
        int[] charsCount = new int[26];
        for (char c : chars) {
            charsCount[c - 'a'] += 1;
        }
        Stack<Character> stack = new Stack<>();
        boolean[] isInStack = new boolean[26];
        for (char c : chars) {
            charsCount[c - 'a'] -= 1;
            if (isInStack[c - 'a']) {
                continue;
            }
            while (!stack.isEmpty() && c < stack.peek() && charsCount[stack.peek() - 'a'] > 0) {
                isInStack[stack.pop() - 'a'] = false;
            }
            stack.push(c);
            isInStack[c - 'a'] = true;
        }
        Collections.reverse(stack);
        char[] result = new char[stack.size()];
        for (int i = 0; i < result.length; i++) {
            result[i] = stack.pop();
        }
        return new String(result);
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        System.out.println(solution.removeDuplicateLetters("cdadabcc").equals("adbc"));
        System.out.println(solution.removeDuplicateLetters("abcd").equals("abcd"));
        System.out.println(solution.removeDuplicateLetters("ecbacba").equals("eacb"));
        System.out.println(solution.removeDuplicateLetters("leetcode").equals("letcod"));
    }
}
