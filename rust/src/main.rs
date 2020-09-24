mod helper;

struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1.eq("0") || num2.eq("0") {
            return "0".to_string();
        }
        let num1: Vec<i32> = num1.as_bytes().iter().map(|&b| (b - 48) as i32).collect();
        let num2: Vec<i32> = num2.as_bytes().iter().map(|&b| (b - 48) as i32).collect();
        fn multiply_single(num: &Vec<i32>, d: i32) -> Vec<i32> {
            if d == 0 {
                return vec![0];
            }
            let mut result = Vec::with_capacity(num.len() + 1);
            let mut carry = 0;
            for i in (0..num.len()).rev() {
                let x = num[i] * d + carry;
                result.push(x % 10);
                carry = x / 10;
            }
            if carry != 0 {
                result.push(carry);
            }
            result.reverse();
            result
        }
        fn add(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
            let l1 = a.len() as i32;
            let l2 = b.len() as i32;
            let mut result = Vec::with_capacity(std::cmp::max(l1, l2) as usize + 1);
            let mut i = l1 - 1;
            let mut j = l2 - 1;
            let mut carry = 0;
            while i >= 0 || j >= 0 {
                let x = if i >= 0 { a[i as usize] } else { 0 };
                let y = if j >= 0 { b[j as usize] } else { 0 };
                let z = x + y + carry;
                result.push(z % 10);
                carry = z / 10;
                i -= 1;
                j -= 1;
            }
            if carry != 0 {
                result.push(carry);
            }
            result.reverse();
            result
        }
        let mut result = vec![];
        for i in (0..num2.len()).rev() {
            let mut x = multiply_single(&num1, num2[i]);
            for _j in 0..num2.len() - i - 1 {
                x.push(0);
            }
            result = add(&result, &x);
        }
        result.iter().map(|&i| std::char::from_digit(i as u32, 10).unwrap()).collect()
    }
}

fn main() {
    assert_eq!("520".to_string(), Solution::multiply("10".to_string(), "52".to_string()));
    assert_eq!("0".to_string(), Solution::multiply("0".to_string(), "52".to_string()));
    assert_eq!("100899".to_string(), Solution::multiply("999".to_string(), "101".to_string()));
    assert_eq!("9990".to_string(), Solution::multiply("999".to_string(), "10".to_string()));
    assert_eq!("0".to_string(), Solution::multiply("999".to_string(), "0".to_string()));
    assert_eq!("8991".to_string(), Solution::multiply("999".to_string(), "9".to_string()));
    assert_eq!("56088".to_string(), Solution::multiply("123".to_string(), "456".to_string()));
    assert_eq!("6".to_string(), Solution::multiply("2".to_string(), "3".to_string()));
}
