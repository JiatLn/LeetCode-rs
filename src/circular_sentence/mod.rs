/// <https://leetcode.cn/problems/circular-sentence/>
struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut words = sentence.split(' ').collect::<Vec<&str>>();
        let first = words[0];
        words.push(first);
        for i in 0..words.len() - 1 {
            if words[i].chars().last() != words[i + 1].chars().nth(0) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::circular_sentence::Solution;

    #[test]
    fn test_is_circular_sentence() {
        let ans = Solution::is_circular_sentence("leetcode exercises sound delightful".to_string());
        assert_eq!(ans, true);

        let ans = Solution::is_circular_sentence("Leetcode is cool".to_string());
        assert_eq!(ans, false);

        assert_eq!(Some(1), Some(1));
    }
}
