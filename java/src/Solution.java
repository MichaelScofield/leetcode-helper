import java.util.Comparator;
import java.util.PriorityQueue;

public class Solution {

    static class MedianFinder {

        double median;
        int total = 0;
        PriorityQueue<Integer> minHeap;
        PriorityQueue<Integer> maxHeap;

        public MedianFinder() {
            minHeap = new PriorityQueue<>(Comparator.naturalOrder());
            maxHeap = new PriorityQueue<>(Comparator.reverseOrder());
        }

        public void addNum(int num) {
            if (total % 2 == 0) {
                minHeap.add(num);
                Integer min = minHeap.poll();
                maxHeap.add(min);

                //noinspection ConstantConditions
                median = maxHeap.peek();
            } else {
                maxHeap.add(num);
                Integer max = maxHeap.poll();
                minHeap.add(max);

                //noinspection ConstantConditions
                median = (minHeap.peek() + (maxHeap.peek() == null ? 0 : maxHeap.peek())) / 2.0;
            }
            total += 1;
        }

        public double findMedian() {
            return median;
        }
    }

    public static void main(String[] args) {
        MedianFinder finder = new MedianFinder();
        finder.addNum(1);
        finder.addNum(2);
        System.out.println(finder.findMedian() == 1.5);
        finder.addNum(3);
        System.out.println(finder.findMedian() == 2);
    }
}
