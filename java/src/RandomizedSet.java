import java.util.*;

class RandomizedSet {

    final Map<Integer, Integer> indices;
    final List<Integer> values;

    public RandomizedSet() {
        indices = new HashMap<>();
        values = new ArrayList<>();
    }

    public boolean insert(int val) {
        if (indices.containsKey(val)) {
            return false;
        }
        values.add(val);
        indices.put(val, values.size() - 1);
        return true;
    }

    public boolean remove(int val) {
        Integer index = indices.get(val);
        if (index == null) {
            return false;
        }
        if (index < values.size() - 1) {
            Integer lastVal = values.get(values.size() - 1);
            values.set(index, lastVal);
            indices.put(lastVal, index);
        }
        // O(1) if remove the last index, see method "fastRemove" in ArrayList
        values.remove(values.size() - 1);
        indices.remove(val);
        return true;
    }

    public int getRandom() {
        Random random = new Random();
        return values.get(random.nextInt(values.size()));
    }

    public static void main(String[] args) {
        RandomizedSet randomSet = new RandomizedSet();
        System.out.println(randomSet.insert(1));
        System.out.println(!randomSet.remove(2));
        System.out.println(randomSet.insert(2));
        System.out.println(randomSet.getRandom());
        System.out.println(randomSet.remove(1));
        System.out.println(!randomSet.insert(2));
        System.out.println(randomSet.getRandom() == 2);
    }
}
