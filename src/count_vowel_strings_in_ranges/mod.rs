use std::collections::HashSet;

/// <https://leetcode.cn/problems/count-vowel-strings-in-ranges/>
struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let vowel_set = HashSet::from([b'a', b'e', b'i', b'o', b'u']);
        let is_vowel_string = |a_str: &str| {
            let ch = a_str.as_bytes();
            vowel_set.contains(&ch[0]) && vowel_set.contains(&ch.last().unwrap())
        };
        let mut cnt = 0;
        let mut cnt_vec = Vec::with_capacity(words.len());
        for word in words.iter() {
            if is_vowel_string(&word) {
                cnt = cnt + 1;
            }
            cnt_vec.push(cnt);
        }
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries.iter() {
            let start_q = q[0] as usize;
            let end_q = q[1] as usize;
            if start_q == 0 {
                ans.push(cnt_vec[end_q]);
            } else {
                ans.push(cnt_vec[end_q] - cnt_vec[start_q - 1]);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::count_vowel_strings_in_ranges::Solution;
    #[test]
    fn test_vowel_strings() {
        let res = Solution::vowel_strings(
            vec![
                "aba".to_string(),
                "bcb".to_string(),
                "ece".to_string(),
                "aa".to_string(),
                "e".to_string()
            ],
            vec![vec![0, 2], vec![1, 4], vec![1, 1]]
        );
        assert_eq!(res, vec![2, 3, 0]);

        let res = Solution::vowel_strings(
            vec!["a".to_string(), "e".to_string(), "i".to_string()],
            vec![vec![0, 2], vec![0, 1], vec![2, 2]]
        );
        assert_eq!(res, vec![3, 2, 1]);
    }
}
