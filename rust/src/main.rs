mod helper;

struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        assert!(s.len() >= 1);

        use std::collections::HashMap;
        use std::collections::HashSet;
        fn partition0<'a>(
            s: &'a [u8],
            cache: &mut HashMap<&'a [u8], HashSet<Vec<String>>>,
        ) -> HashSet<Vec<String>> {
            if let Some(ans) = cache.get(s) {
                return ans.clone();
            }
            let mut ans = HashSet::new();
            let mut i = 0;
            while i < s.len() {
                let mut palindromes = Vec::with_capacity(2);
                let mut l = i;
                let mut r = i;
                while l >= 1 && r <= s.len() - 2 && s[l - 1] == s[r + 1] {
                    l -= 1;
                    r += 1;
                }
                palindromes.push((l, r));

                if i > 0 && s[i - 1] == s[i] {
                    let mut l = i - 1;
                    let mut r = i;
                    while l >= 1 && r <= s.len() - 2 && s[l - 1] == s[r + 1] {
                        l -= 1;
                        r += 1;
                    }
                    palindromes.push((l, r));
                }

                for (l, r) in palindromes {
                    let left_parts = partition0(&s[0..l], cache);
                    let right_parts = partition0(&s[r + 1..], cache);
                    if !left_parts.is_empty() {
                        if !right_parts.is_empty() {
                            for lp in left_parts.iter() {
                                for rp in right_parts.iter() {
                                    let mut v = vec![];
                                    v.append(&mut lp.clone());
                                    v.push(String::from_utf8(s[l..=r].to_vec()).unwrap());
                                    v.append(&mut rp.clone());
                                    ans.insert(v);
                                }
                            }
                        } else {
                            for lp in right_parts.iter() {
                                let mut v = vec![];
                                v.append(&mut lp.clone());
                                v.push(String::from_utf8(s[l..=r].to_vec()).unwrap());
                                ans.insert(v);
                            }
                        }
                    } else {
                        if !right_parts.is_empty() {
                            for rp in right_parts.iter() {
                                let mut v = vec![];
                                v.push(String::from_utf8(s[l..=r].to_vec()).unwrap());
                                v.append(&mut rp.clone());
                                ans.insert(v);
                            }
                        } else {
                            let mut v = vec![];
                            v.push(String::from_utf8(s[l..=r].to_vec()).unwrap());
                            ans.insert(v);
                        }
                    }
                }

                i += 1;
            }
            cache.insert(s, ans.clone());
            ans
        }
        let mut cache = HashMap::new();
        partition0(s.as_bytes(), &mut cache).into_iter().collect()
    }
}

fn main() {
    println!("{:?}", Solution::partition("fffffffffffffff".to_string()));
    println!("{:?}", Solution::partition("aabaa".to_string()));
    println!("{:?}", Solution::partition("aba".to_string()));
    println!("{:?}", Solution::partition("aab".to_string()));
    assert_eq!(vecvec![["a"]], Solution::partition("a".to_string()));
}
