/// <https://leetcode.cn/problems/minimum-cuts-to-divide-a-circle/>
struct Solution;

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 || (n & 1) == 0 { n >> 1 } else { n }
    }
}

#[cfg(test)]
mod tests {
    use crate::minimum_cuts_to_divide_a_circle::Solution;

    #[test]
    fn test_number_of_cuts() {
        let ans = Solution::number_of_cuts(4);
        assert_eq!(ans, 2);

        let ans = Solution::number_of_cuts(3);
        assert_eq!(ans, 3);

        let ans = Solution::number_of_cuts(1);
        assert_eq!(ans, 0);
    }
}
