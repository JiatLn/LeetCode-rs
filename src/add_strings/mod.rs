/// <https://leetcode.cn/problems/add-strings/>
struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut ans = vec![];
        let mut l1 = num1.len();
        let mut l2 = num2.len();
        let mut num3 = 0;
        while l1 > 0 || l2 > 0 || num3 > 0 {
            let mut val = num3;
            if l1 > 0 {
                val += num1.as_bytes()[l1 - 1] - "0".as_bytes()[0];
                l1 -= 1;
            }
            if l2 > 0 {
                val += num2.as_bytes()[l2 - 1] - "0".as_bytes()[0];
                l2 -= 1;
            }
            if val > 9 {
                num3 = 1;
            } else {
                num3 = 0;
            }
            ans.push((val % 10).to_string());
        }
        ans.reverse();
        ans.join("")
    }
}

#[cfg(test)]
mod tests {
    use crate::add_strings::Solution;

    #[test]
    fn test_add_strings() {
        let ans = Solution::add_strings("11".to_string(), "123".to_string());
        assert_eq!(ans, "134".to_string());
    }
}
