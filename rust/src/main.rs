mod helper;

struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let version1: Vec<&str> = version1.split(".").collect();
        let version2: Vec<&str> = version2.split(".").collect();
        let mut i = 0;
        let mut j = 0;
        while i < version1.len() || j < version2.len() {
            let v1 = version1
                .get(i)
                .map(|x| x.parse::<i32>().unwrap())
                .unwrap_or(0);
            let v2 = version2
                .get(j)
                .map(|x| x.parse::<i32>().unwrap())
                .unwrap_or(0);
            if v1 < v2 {
                return -1;
            }
            if v1 > v2 {
                return 1;
            }
            i += 1;
            j += 1;
        }
        0
    }
}

fn main() {
    assert_eq!(
        0,
        Solution::compare_version("1.01".to_string(), "1.001".to_string())
    );
    assert_eq!(
        0,
        Solution::compare_version("1.0".to_string(), "1.0.0".to_string())
    );
    assert_eq!(
        -1,
        Solution::compare_version("0.1".to_string(), "1.1".to_string())
    );
    assert_eq!(
        1,
        Solution::compare_version("1.0.1".to_string(), "1".to_string())
    );
    assert_eq!(
        -1,
        Solution::compare_version("7.5.2.4".to_string(), "7.5.3".to_string())
    );
}
