mod helper;

struct Solution;

impl Solution {
    pub fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        assert!(arr.len() > 0);
        fn flip(arr: &mut [i32], n: usize) {
            let mut i = 0;
            let mut j = n;
            while i < j {
                arr.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        fn pancake_sort(arr: &mut [i32], flips: &mut Vec<i32>) {
            let len = arr.len();
            if len == 0 {
                return;
            }
            let max_index = arr.iter().enumerate().max_by_key(|e| e.1).unwrap().0;
            if max_index < len - 1 {
                if max_index > 0 {
                    flip(arr, max_index);
                    flips.push(max_index as i32 + 1);
                }
                flip(arr, len - 1);
                flips.push(len as i32);
            }
            pancake_sort(&mut arr[..len - 1], flips)
        }
        let arr = &mut { arr };
        let mut flips = Vec::with_capacity(arr.len());
        pancake_sort(arr.as_mut_slice(), &mut flips);
        flips
    }
}

fn main() {
    assert_eq!(vec![3, 4, 2, 3, 2], Solution::pancake_sort(vec![3, 2, 4, 1]));
    assert_eq!(Vec::<i32>::new(), Solution::pancake_sort(vec![1, 2, 3]));
}
