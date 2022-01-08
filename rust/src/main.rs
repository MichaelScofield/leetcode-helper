mod helper;

struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        assert_ne!(denominator, 0);
        if numerator == 0 {
            return "0".to_string();
        }
        if numerator == i32::MIN && denominator == -1 {
            return "2147483648".to_string();
        }

        let mut digits = vec![];
        let is_negative = numerator > 0 && denominator < 0 || numerator < 0 && denominator > 0;
        if is_negative {
            digits.push('-');
        }

        let mut x = (numerator as i64).abs();
        let y = (denominator as i64).abs();
        for c in (x / y).to_string().chars() {
            digits.push(c);
        }
        let mut r = x % y;
        if r == 0 {
            return digits.iter().collect();
        }
        digits.push('.');

        let mut ds = vec![];
        let mut xs = vec![];
        loop {
            x = r * 10;
            r = x % y;

            let mut i = 0;
            while i < xs.len() {
                if xs[i] == x {
                    break;
                }
                i += 1;
            }
            if i < xs.len() {
                ds.insert(i, '(');
                ds.push(')');
                break;
            }

            let d = x / y;
            ds.push(char::from_digit(d.abs() as u32, 10).unwrap());
            xs.push(x);

            if r == 0 {
                break;
            }
        }
        digits.append(&mut ds);
        digits.iter().collect()
    }
}

fn main() {
    assert_eq!(
        "-0.58(3)".to_string(),
        Solution::fraction_to_decimal(7, -12)
    );
    assert_eq!(
        "0.0000000004656612873077392578125".to_string(),
        Solution::fraction_to_decimal(-1, -2147483648)
    );
    assert_eq!("-6.25".to_string(), Solution::fraction_to_decimal(-50, 8));
    assert_eq!("0.125".to_string(), Solution::fraction_to_decimal(1, 8));
    assert_eq!("0.1(6)".to_string(), Solution::fraction_to_decimal(1, 6));
    assert_eq!("0.(012)".to_string(), Solution::fraction_to_decimal(4, 333));
    assert_eq!("0.5".to_string(), Solution::fraction_to_decimal(1, 2));
    assert_eq!("2".to_string(), Solution::fraction_to_decimal(2, 1));
    assert_eq!("0.(6)".to_string(), Solution::fraction_to_decimal(2, 3));
    assert_eq!("0.2".to_string(), Solution::fraction_to_decimal(1, 5));
}
