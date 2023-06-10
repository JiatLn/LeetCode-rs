/// <https://leetcode.cn/problems/compare-strings-by-frequency-of-the-smallest-character/>
struct Solution;

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let words = words
            .iter()
            .map(|word| {
                word.chars()
                    .filter(|&c| c == word.chars().min().unwrap())
                    .count()
            })
            .collect::<Vec<usize>>();
        queries
            .iter()
            .map(|query| {
                let cnt = query
                    .chars()
                    .filter(|&c| c == query.chars().min().unwrap())
                    .count();
                words
                    .iter()
                    .filter(|&&w| cnt < w)
                    .count() as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::compare_strings_by_frequency_of_the_smallest_character::Solution;

    #[test]
    fn test_num_smaller_by_frequency() {
        let queries = vec!["cbd".to_string()];
        let words = vec!["zaaaz".to_string()];
        let ans = Solution::num_smaller_by_frequency(queries, words);
        assert_eq!(ans, vec![1]);

        let queries = vec!["bbb".to_string(), "cc".to_string()];
        let words = vec!["a".to_string(), "aa".to_string(), "aaa".to_string(), "aaaa".to_string()];
        let ans = Solution::num_smaller_by_frequency(queries, words);
        assert_eq!(ans, vec![1, 2]);
    }
}
