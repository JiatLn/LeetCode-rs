/// <https://leetcode.cn/problems/k-items-with-the-maximum-sum/>
struct Solution;

impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        _num_neg_ones: i32,
        k: i32
    ) -> i32 {
        if k <= num_ones + num_zeros {
            k.min(num_ones)
        } else {
            // num_ones - (k - num_ones - num_zeros)
            num_ones * 2 + num_zeros - k
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::k_items_with_the_maximum_sum::Solution;

    #[test]
    fn test_k_items_with_maximum_sum() {
        let ans = Solution::k_items_with_maximum_sum(3, 2, 0, 2);
        assert_eq!(ans, 2);

        let ans = Solution::k_items_with_maximum_sum(3, 2, 0, 4);
        assert_eq!(ans, 3);

        let ans = Solution::k_items_with_maximum_sum(3, 2, 3, 6);
        assert_eq!(ans, 2);
    }
}
