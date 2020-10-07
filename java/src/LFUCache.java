import java.util.HashMap;
import java.util.Map;
import java.util.Objects;
import java.util.TreeSet;

class LFUCache {

    static class Freq {

        final int key;
        int value;
        int used;
        long lastVisitTime;

        Freq(int key, int value) {
            this.key = key;
            this.value = value;
            visit();
        }

        void visit() {
            used += 1;
            lastVisitTime = System.nanoTime();
        }

        @Override
        public boolean equals(Object o) {
            assert o instanceof Freq;
            Freq freq = (Freq) o;
            return used == freq.used && lastVisitTime == freq.lastVisitTime;
        }

        @Override
        public int hashCode() {
            return Objects.hash(used, lastVisitTime);
        }

        @Override
        public String toString() {
            return "Freq{" +
                    "key=" + key +
                    ", value=" + value +
                    ", used=" + used +
                    ", lastVisitTime=" + lastVisitTime +
                    '}';
        }
    }

    final Map<Integer, Freq> cache;
    final TreeSet<Freq> freqs;
    final int capacity;

    public LFUCache(int capacity) {
        cache = new HashMap<>();
        freqs = new TreeSet<>((o1, o2) -> {
            int t = o1.used - o2.used;
            return t == 0 ? (int) (o1.lastVisitTime - o2.lastVisitTime) : t;
        });
        this.capacity = capacity;
    }

    public int get(int key) {
        Freq freq = cache.get(key);
        if (freq == null) {
            return -1;
        }
        freqs.remove(freq);
        freq.visit();
        freqs.add(freq);
        return freq.value;
    }

    public void put(int key, int value) {
        if (capacity <= 0) {
            return;
        }
        Freq freq = cache.get(key);
        if (freq != null) {
            freqs.remove(freq);
            freq.value = value;
            freq.visit();
        } else {
            if (cache.size() >= capacity) {
                Freq leastUsed = freqs.pollFirst();
                assert leastUsed != null;
                cache.remove(leastUsed.key);
            }
            freq = new Freq(key, value);
            cache.put(key, freq);
        }
        freqs.add(freq);
    }

    public static void main(String[] args) {
        LFUCache cache;

        cache = new LFUCache(2);
        cache.put(1, 1);
        cache.put(2, 2);
        System.out.println(cache.get(1) == 1);
        cache.put(3, 3);
        System.out.println(cache.get(2) == -1);
        System.out.println(cache.get(3) == 3);
        cache.put(4, 4);
        System.out.println(cache.get(1) == -1);
        System.out.println(cache.get(3) == 3);
        System.out.println(cache.get(4) == 4);

        cache = new LFUCache(0);
        cache.put(0, 0);
        System.out.println(cache.get(0) == -1);
    }
}
