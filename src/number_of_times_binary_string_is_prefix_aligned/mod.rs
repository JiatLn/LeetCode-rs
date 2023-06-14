/// <https://leetcode.cn/problems/number-of-times-binary-string-is-prefix-aligned/>
struct Solution;

impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut right = 0;
        flips
            .iter()
            .enumerate()
            .for_each(|(i, &flip)| {
                right = right.max(flip);
                if right == (i as i32) + 1 {
                    ans += 1;
                }
            });
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::number_of_times_binary_string_is_prefix_aligned::Solution;
    #[test]
    fn test_num_times_all_blue() {
        let ans = Solution::num_times_all_blue(vec![3, 2, 4, 1, 5]);
        assert_eq!(ans, 2);

        let ans = Solution::num_times_all_blue(vec![4, 1, 2, 3]);
        assert_eq!(ans, 1);

        let ans = Solution::num_times_all_blue(vec![2, 1, 3, 5, 4]);
        assert_eq!(ans, 3);
    }
}
