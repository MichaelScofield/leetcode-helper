mod helper;

struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        assert!(s.len() >= 1);

        #[derive(PartialEq)]
        enum Number {
            Decimal,
            Integer,
        }

        fn is_number(chars: &[char]) -> Option<Number> {
            #[derive(PartialEq)]
            enum ParsingState {
                CheckSign,
                CheckInteger,
                CheckDecimal,
            }
            let mut parsing_state = ParsingState::CheckSign;

            let mut has_digit = false;
            for c in chars {
                match *c {
                    '+' | '-' => {
                        if parsing_state != ParsingState::CheckSign {
                            return None;
                        }
                        parsing_state = ParsingState::CheckInteger;
                    }
                    _ if c.is_digit(10) => {
                        if parsing_state == ParsingState::CheckSign {
                            parsing_state = ParsingState::CheckInteger;
                        }
                        has_digit = true;
                    }
                    '.' => {
                        if parsing_state == ParsingState::CheckDecimal {
                            return None;
                        }
                        parsing_state = ParsingState::CheckDecimal;
                    }
                    _ => return None,
                }
            }
            if !has_digit {
                return None;
            }

            match parsing_state {
                ParsingState::CheckInteger => Some(Number::Integer),
                ParsingState::CheckDecimal => Some(Number::Decimal),
                _ => None,
            }
        }

        let chars = s.chars().collect::<Vec<char>>();
        let mut split = chars.splitn(2, |&x| x == 'e' || x == 'E');
        if let Some(number_part) = split.next() {
            if is_number(number_part) == None {
                return false;
            }
        } else {
            return false;
        }

        if let Some(exp_part) = split.next() {
            if is_number(exp_part) != Some(Number::Integer) {
                return false;
            }
        }
        true
    }
}

fn main() {
    assert!(!Solution::is_number(".".to_string()));
    assert!(!Solution::is_number("1e2e3".to_string()));
    assert!(!Solution::is_number("1-2".to_string()));
    assert!(Solution::is_number("0".to_string()));
    assert!(!Solution::is_number("e".to_string()));
    assert!(Solution::is_number(".1".to_string()));
    assert!(Solution::is_number("2".to_string()));
    assert!(Solution::is_number("0089".to_string()));
    assert!(Solution::is_number("-0.1".to_string()));
    assert!(Solution::is_number("+3.14".to_string()));
    assert!(Solution::is_number("4.".to_string()));
    assert!(Solution::is_number("-.9".to_string()));
    assert!(Solution::is_number("2e10".to_string()));
    assert!(Solution::is_number("-90E3".to_string()));
    assert!(Solution::is_number("3e+7".to_string()));
    assert!(Solution::is_number("+6e-1".to_string()));
    assert!(Solution::is_number("53.5e93".to_string()));
    assert!(Solution::is_number("-123.456e789".to_string()));
    assert!(!Solution::is_number("abc".to_string()));
    assert!(!Solution::is_number("1a".to_string()));
    assert!(!Solution::is_number("1e".to_string()));
    assert!(!Solution::is_number("e3".to_string()));
    assert!(!Solution::is_number("99e2.5".to_string()));
    assert!(!Solution::is_number("--6".to_string()));
    assert!(!Solution::is_number("95a54e53".to_string()));
}
