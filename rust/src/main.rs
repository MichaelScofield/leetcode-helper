mod helper;

struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() == 0 {
            return s2 == s3;
        }
        if s2.len() == 0 {
            return s1 == s3;
        }
        if s3.len() == 0 {
            return s1 == "" && s2 == "";
        }
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        fn f(s: &str) -> Vec<Vec<String>> {
            if s.len() == 1 {
                return vec![vec![s.to_string()]];
            }

            let mut partitions = vec![];
            let remaining_partitions = f(&s[1..]);

            for p in remaining_partitions.iter() {
                let mut v = vec![];
                v.push(String::from(&s[0..1]));
                v.append(&mut p.iter().map(String::from).collect());
                partitions.push(v);
            }

            for p in remaining_partitions.iter() {
                let mut v = vec![];

                let mut first = String::new();
                first.push_str(&s[0..1]);
                first.push_str(p[0].as_str());
                v.push(first);

                v.append(&mut p[1..].iter().map(String::from).collect());
                partitions.push(v);
            }
            partitions
        }
        let partitions = f(s3.as_str());

        for partition in partitions {
            let mut a = String::new();
            let mut b = String::new();
            for i in 0..partition.len() {
                if i % 2 == 0 {
                    a.push_str(&partition[i]);
                } else {
                    b.push_str(&partition[i]);
                }
            }
            if a == s1 && b == s2 || a == s2 && b == s1 {
                return true;
            }
        }
        false
    }
}

fn main() {
    assert!(!Solution::is_interleave(
        "abaaacbacaab".to_string(),
        "bcccababccc".to_string(),
        "bcccabaaaaabccaccbacabb".to_string()
    ));
    assert!(!Solution::is_interleave(
        "".to_string(),
        "abc".to_string(),
        "abcd".to_string()
    ));
    assert!(Solution::is_interleave(
        "aabcc".to_string(),
        "dbbca".to_string(),
        "aadbbcbcac".to_string()
    ));
    assert!(!Solution::is_interleave(
        "aabcc".to_string(),
        "dbbca".to_string(),
        "aadbbbaccc".to_string()
    ));
    assert!(Solution::is_interleave(
        "".to_string(),
        "".to_string(),
        "".to_string()
    ));
}
