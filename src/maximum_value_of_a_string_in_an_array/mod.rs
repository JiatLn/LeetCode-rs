/// <https://leetcode.cn/problems/maximum-value-of-a-string-in-an-array/>
struct Solution;

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        // strs.iter()
        //     .map(|str| str.parse::<i32>().unwrap_or(str.len() as i32))
        //     .max()
        //     .unwrap()
        strs.iter().fold(0, |a, b| a.max(b.parse().unwrap_or(b.len() as i32)))
    }
}

#[cfg(test)]
mod tests {
    use crate::maximum_value_of_a_string_in_an_array::Solution;

    #[test]
    fn test_maximum_value() {
        let ans = Solution::maximum_value(
            vec![
                "alic3".to_string(),
                "bob".to_string(),
                "3".to_string(),
                "4".to_string(),
                "00000".to_string()
            ]
        );
        assert_eq!(ans, 5);

        let ans = Solution::maximum_value(
            vec!["1".to_string(), "01".to_string(), "001".to_string(), "0001".to_string()]
        );
        assert_eq!(ans, 1)
    }
}
